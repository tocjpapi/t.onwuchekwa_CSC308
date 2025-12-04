use std::process::Command;

fn main() {
    let output = Command::new("ls")
                .arg("-la")
                .output()
                .expect("failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}