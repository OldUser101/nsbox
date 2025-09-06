use nix::sched::CloneFlags;

#[derive(Default, Copy, Clone)]
pub struct Config {
    pub namespaces: NamespacesConfig,
}

#[derive(Copy, Clone)]
pub struct NamespacesConfig {
    pub new_mount: bool,
    pub new_pid: bool,
    pub new_net: bool,
    pub new_ipc: bool,
    pub new_uts: bool,
    pub new_user: bool,
    pub new_cgroup: bool,
}

impl Default for NamespacesConfig {
    fn default() -> Self {
        Self {
            new_mount: false,
            new_pid: false,
            new_net: false,
            new_ipc: false,
            new_uts: false,
            new_user: true,
            new_cgroup: false,
        }
    }
}

impl From<NamespacesConfig> for CloneFlags {
    fn from(cfg: NamespacesConfig) -> CloneFlags {
        let mut flags = CloneFlags::empty();

        if cfg.new_mount {
            flags.insert(CloneFlags::CLONE_NEWNS);
        }

        if cfg.new_pid {
            flags.insert(CloneFlags::CLONE_NEWPID);
        }

        if cfg.new_net {
            flags.insert(CloneFlags::CLONE_NEWNET);
        }

        if cfg.new_ipc {
            flags.insert(CloneFlags::CLONE_NEWIPC);
        }

        if cfg.new_uts {
            flags.insert(CloneFlags::CLONE_NEWUTS);
        }

        if cfg.new_user {
            flags.insert(CloneFlags::CLONE_NEWUSER);
        }

        if cfg.new_cgroup {
            flags.insert(CloneFlags::CLONE_NEWCGROUP);
        }

        flags
    }
}
