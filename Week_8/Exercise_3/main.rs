use std::process::{Command, Child};
use std::{thread, time::Duration};

fn main() {
    // Spawn a long-running process: ping google.com
    let mut child: Child = Command::new("ping")
        .arg("google.com")
        .spawn()
        .expect("Failed to start ping");

    println!("Ping started with PID: {}", child.id());
    println!("Check 'top' in another terminal to see it running...");

    // Wait for 5 seconds
    thread::sleep(Duration::from_secs(5));

    // Kill the child process
    match child.kill() {
        Ok(_) => println!("Ping process killed!"),
        Err(e) => println!("Failed to kill process: {}", e),
    }

    // Wait for the child to exit and capture the exit status
    let exit_status = child.wait().expect("Failed to wait for child");
    println!("Exit status: {}", exit_status);
}
