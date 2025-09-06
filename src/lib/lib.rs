pub mod config;

pub use config::{Config, NamespacesConfig};

use nix::{
    sched::{CloneCb, clone},
    sys::wait::{WaitStatus, waitpid},
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

    pub fn run<F>(&mut self, mut f: F) -> Result<i32, String>
    where
        F: FnMut() -> isize + 'static,
    {
        let mut stack = vec![0u8; 1024 * 1024];

        let cb: CloneCb = Box::new(move || f());

        let pid = unsafe {
            clone(
                cb,
                &mut stack,
                self.config.namespaces.into(),
                Some(nix::libc::SIGCHLD), // Pass SIGCHLD here, otherwise `waitpid` fails
            )
        }
        .map_err(|e| e.to_string())?;

        let status = waitpid(pid, None).map_err(|e| e.to_string())?;

        if let WaitStatus::Exited(_, code) = status {
            Ok(code)
        } else {
            Err("Process did not exit properly".to_string())
        }
    }
}
