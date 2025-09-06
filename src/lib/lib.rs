pub mod config;
pub mod namespaces;

pub use config::Config;
pub use namespaces::NamespacesConfig;

use nix::{
    sched::{CloneCb, clone},
    sys::wait::{WaitStatus, waitpid},
    unistd::{Pid, pipe, read, write},
};
use std::{
    fs::OpenOptions,
    io::Write,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
};

pub struct Sandbox {
    config: Config,
}

impl Sandbox {
    pub fn new(config: Option<Config>) -> Self {
        Self {
            config: config.unwrap_or_default(),
        }
    }

    fn write_uid_map(
        &self,
        pid: Pid,
        uid_map: (u32, u32, u32),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let uid_map_path = format!("/proc/{}/uid_map", pid.as_raw());
        let uid_map_entry = format!("{} {} {}", uid_map.0, uid_map.1, uid_map.2);
        let mut uid_map_file = OpenOptions::new().write(true).open(&uid_map_path)?;
        uid_map_file.write_all(uid_map_entry.as_bytes())?;

        Ok(())
    }

    fn write_deny_setgroups(&self, pid: Pid) -> Result<(), Box<dyn std::error::Error>> {
        let setgroups_path = format!("/proc/{}/setgroups", pid.as_raw());
        let mut setgroups_file = OpenOptions::new().write(true).open(&setgroups_path)?;
        setgroups_file.write_all(b"deny")?;

        Ok(())
    }

    fn write_gid_map(
        &self,
        pid: Pid,
        gid_map: (u32, u32, u32),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gid_map_path = format!("/proc/{}/gid_map", pid.as_raw());
        let gid_map_entry = format!("{} {} {}", gid_map.0, gid_map.1, gid_map.2);
        let mut gid_map_file = OpenOptions::new().write(true).open(&gid_map_path)?;
        gid_map_file.write_all(gid_map_entry.as_bytes())?;

        Ok(())
    }

    fn write_user_ns_maps(&self, pid: Pid) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.namespaces.new_user {
            if let Some(uid_map) = self.config.namespaces.user_config.uid_map {
                self.write_uid_map(pid, uid_map)?;
            }
            if let Some(gid_map) = self.config.namespaces.user_config.gid_map {
                // Make sure to write the `setgroups` file!
                self.write_deny_setgroups(pid)?;
                self.write_gid_map(pid, gid_map)?;
            }
        }

        Ok(())
    }

    pub fn run<F>(&mut self, mut f: F) -> Result<i32, String>
    where
        F: FnMut() -> isize + 'static,
    {
        let mut stack = vec![0u8; 1024 * 1024];
        let (read_pipe, write_pipe) = pipe().map_err(|e| e.to_string())?;
        let raw_read_pipe = read_pipe.as_raw_fd();

        let cb: CloneCb = Box::new(move || {
            // Read a single byte from th pipe, this blocks until parent set-up is complete
            let mut buf = [0u8; 1];
            let _ = read(unsafe { OwnedFd::from_raw_fd(raw_read_pipe) }, &mut buf).unwrap();

            f()
        });

        let pid = unsafe {
            clone(
                cb,
                &mut stack,
                self.config.namespaces.into(),
                Some(nix::libc::SIGCHLD), // Pass SIGCHLD here, otherwise `waitpid` fails
            )
        }
        .map_err(|e| e.to_string())?;

        // This file descriptor is now owned by the other process, drop it
        std::mem::drop(read_pipe);

        self.write_user_ns_maps(pid).map_err(|e| e.to_string())?;

        // Write a byte to signal the child it can start executing
        let buf = [0u8; 1];
        write(write_pipe, &buf).map_err(|e| e.to_string())?;

        match waitpid(pid, None).map_err(|e| e.to_string())? {
            WaitStatus::Exited(_, code) => Ok(code),
            WaitStatus::Signaled(_, signal, _) => Ok(signal as i32),
            _ => Err("Process did not exit properly".to_string()),
        }
    }
}
