//??? REMOVE THIS FILE WHEN MAIN INTEROP ISSUE IS FIXED
use std::process::{Command, Stdio};
use execute::{Execute, command};

pub fn exectythepy() {
    let mut command1 = command("cd './pyDumpImg");
    let command2 = command("python -u './pyDumpImg/main.py'");

    let mut child = command1
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let output = child 
        .wait_with_output()
        .expect("failed to wait on child");

    println!("status: {}", output.status);
}