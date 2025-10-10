extern crate nsbox;

use nix::unistd::{getgid, getpid, getuid};
use nsbox::{Config, MountNamespaceConfig, NamespacesConfig, Sandbox};

#[test]
fn test_pid_namespace() {
    let cfg = Config {
        namespaces: NamespacesConfig {
            new_pid: true,
            ..Default::default()
        },
    };

    let mut sandbox = Sandbox::new(Some(cfg));

    let result = sandbox
        .run(|| {
            let pid = getpid();
            println!("PID inside sandbox: {pid}");

            if pid.as_raw() != 1 {
                eprintln!("PID namespace test failed!");
                1
            } else {
                0
            }
        })
        .expect("Sandbox failed");

    println!("Sandbox exited with code: {result}");
    assert_eq!(result, 0);
}

#[test]
fn test_user_namespace() {
    // `namespaces.new_user` should be set by default
    let mut sandbox = Sandbox::new(None);

    let result = sandbox
        .run(|| {
            let uid = getuid();
            let gid = getgid();
            println!("UID, GID inside sandbox: {uid}, {gid}");

            // Inside the sandbox, the UID and GID should both be 0
            if uid.as_raw() != 0 || gid.as_raw() != 0 {
                eprintln!("User namespace test failed!");
                1
            } else {
                0
            }
        })
        .expect("Sandbox failed");

    println!("Sandbox exited with code: {result}");
    assert_eq!(result, 0);
}

#[test]
fn test_mount_namespace() {
    let cfg = Config {
        namespaces: NamespacesConfig {
            new_mount: true,
            mount_config: MountNamespaceConfig {
                new_root: Some("/tmp/nsbox_test_root".to_string()),
                ..Default::default()
            },
            ..Default::default()
        },
    };

    std::fs::create_dir_all("/tmp/nsbox_test_root").unwrap();

    let mut sandbox = Sandbox::new(Some(cfg));

    let result = sandbox
        .run(|| {
            let cwd = std::env::current_dir().unwrap();
            if cwd != std::path::Path::new("/") {
                1
            } else {
                0
            }
        })
        .expect("Sandbox failed");

    println!("Sandbox exited with code: {result}");
    assert_eq!(result, 0);
}

#[test]
fn test_namespaces_config_defaults() {
    let cfg = NamespacesConfig::default();

    assert!(cfg.new_user);
}
