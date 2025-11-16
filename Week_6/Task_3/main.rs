use std::process::Command;

fn main() {
    let child = Command::new("echo")
        .arg("Hello from child process!")
        .spawn();

    match child {
        Ok(mut handle) => {
            // Wait for the child process to finish
            handle.wait().expect("Failed to wait on child");
        }
        Err(e) => {
            eprintln!("Failed to spawn child process: {}", e);
        }
    }
}
