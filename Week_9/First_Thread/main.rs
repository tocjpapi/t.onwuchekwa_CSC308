use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello from new thread!");
    });

    println!("Hello from the main thread!");
    thread::sleep(std::time::Duration::from_millis(10));
}