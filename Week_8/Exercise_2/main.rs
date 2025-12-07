use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Run the command: echo "Rust Process Management"
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output()
        .expect("Failed to run echo command");

    // Convert stdout from bytes to String
    let stdout_text = String::from_utf8_lossy(&output.stdout);

    // Write the captured output into output.txt
    let mut file = File::create("output.txt")
        .expect("Failed to create file");

    file.write_all(stdout_text.as_bytes())
        .expect("Failed to write to file");

    println!("Written to output.txt: {}", stdout_text);
}
