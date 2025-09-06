extern crate nsbox;

use nix::unistd::execve;
use nsbox::Sandbox;
use std::env::{args, vars_os};
use std::ffi::CString;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: nsbox <COMMAND> [OPTIONS] ...");
        std::process::exit(1);
    }

    let command = CString::new(args[1].clone()).unwrap();
    let arguments: Vec<CString> = args[1..]
        .iter()
        .map(|arg| CString::new(arg.clone()).unwrap())
        .collect();
    let environment: Vec<CString> = vars_os()
        .map(|(k, v)| {
            let s = format!("{}={}", k.to_string_lossy(), v.to_string_lossy());
            CString::new(s).unwrap()
        })
        .collect();

    let mut sandbox = Sandbox::new(None);

    let result = sandbox
        .run(move || {
            match execve(&command, &arguments, &environment) {
                Ok(_) => {}
                Err(e) => eprintln!("execve failed: {}", e),
            }

            1
        })
        .expect("Sandbox failed");

    std::process::exit(result);
}
