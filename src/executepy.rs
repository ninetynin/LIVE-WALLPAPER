//??? REMOVE THIS FILE WHEN MAIN INTEROP ISSUE IS FIXED
use std::process::{Command, Stdio};
use execute::{Execute, command};

pub fn exectythepy() {

    // let cdpath = "./pyDumpImg";
    let mut cdpathnotrelative = "F:\\GITHUB REPOS\\ninetynin\\LIVE-WALLPAPER\\src\\pyDumpImg";

    let mut child = Command::new("python")
        .arg("main.py")
        .current_dir(cdpathnotrelative)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let output = child.wait_with_output().expect("failed to wait on child");

    println!("status: {}", output.status);
}