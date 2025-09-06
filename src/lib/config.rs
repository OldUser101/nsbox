#[derive(Default)]
pub struct Config {
    pub namespaces: NamespacesConfig,
}

#[derive(Default)]
pub struct NamespacesConfig {
    pub new_mount: bool,
    pub new_pid: bool,
    pub new_net: bool,
    pub new_ipc: bool,
    pub new_uts: bool,
    pub new_user: bool,
    pub new_cgroup: bool,
}
