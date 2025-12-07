use std::process::{Command, Stdio};

fn main() {
    // Child 1: sleep 5
    let child1 = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to start sleep");

    println!("Spawned child 1 (sleep 5) with PID: {}", child1.id());

    // Child 2: ls -la
    let child2 = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to start ls -la");

    println!("Spawned child 2 (ls -la) with PID: {}", child2.id());

    // Child 3: echo "Hello from child"
    let child3 = Command::new("bash")
        .arg("-c")
        .arg("echo 'Hello from child'")
        .spawn()
        .expect("Failed to start hello child");

    println!("Spawned child 3 (echo) with PID: {}", child3.id());

    println!("\nNow open a **new terminal** and run:");
    println!("  ps -ef | grep {}", std::process::id());
    println!("or:");
    println!("  pstree -p {}", std::process::id());
    println!("or just: top");
}
