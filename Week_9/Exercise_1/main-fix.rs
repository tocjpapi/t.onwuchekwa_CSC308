use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Shared counter telling which thread is allowed to send next
    let turn = Arc::new(Mutex::new(1)); // T1 goes first

    for i in 1..=3 {
        let tx = tx.clone();
        let turn = Arc::clone(&turn);

        thread::spawn(move || {
            let prefix = format!("T{}:", i);

            for msg in 1..=5 {
                // Wait until it's this thread's turn
                loop {
                    let mut t = turn.lock().unwrap();
                    if *t == i {
                        // It's our turn
                        tx.send(format!("{} message {}", prefix, msg)).unwrap();
                        break;
                    }
                    // Drop lock before retrying
                    drop(t);
                    thread::yield_now();
                }
            }

            // After finishing 5 messages, allow next thread
            let mut t = turn.lock().unwrap();
            *t += 1; // T1 → T2 → T3
        });
    }

    drop(tx);

    for msg in rx {
        println!("{}", msg);
    }
}
