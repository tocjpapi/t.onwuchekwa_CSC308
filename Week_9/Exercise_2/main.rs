use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a channel: tx = sender, rx = receiver
    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];

    for _ in 0..10 {
        let tx_clone = tx.clone();

        // Spawn each thread
        let handle = thread::spawn(move || {
            // Each thread sends 3 to the main thread
            tx_clone.send(3).unwrap();
        });

        handles.push(handle);
    }

    // Drop the original sender so the loop below ends
    drop(tx);

    // Receive all values and add them up
    let mut total = 0;
    for value in rx {
        total += value;
    }

    println!("Final counter value: {}", total);
}
