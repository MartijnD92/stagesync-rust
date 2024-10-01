use std::{path::PathBuf, process::Command};

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

pub fn main() {
    if !create_rust_app::net::is_port_free(5173) {
        println!("========================================================");
        println!(" ViteJS needs to run on");
        println!(" port 5173, but it seems to be in use.");
        println!("========================================================");
        panic!("Port 5173 is taken, but is required for development!")
    }

    let dir = env!("CARGO_MANIFEST_DIR");

    Command::new(NPM)
        .args(["run", "start"])
        .current_dir(PathBuf::from_iter([dir, "frontend"]))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
}
