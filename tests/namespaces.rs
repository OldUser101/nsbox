extern crate nsbox;

use nix::unistd::getpid;
use nsbox::{Config, NamespacesConfig, Sandbox};

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
            println!("PID inside sandbox: {}", pid);

            if pid.as_raw() != 1 {
                eprintln!("PID namespace test failed!");
                1
            } else {
                0
            }
        })
        .expect("Sandbox failed");

    println!("Sandbox exited with code: {}", result);
    assert_eq!(result, 0);
}
