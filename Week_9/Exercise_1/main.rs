use std::sync::mpsc;
use std::thread;

fn main(){
    let (tx, rx) = mpsc::channel();

    for i in 1..=3{
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let prefix = format!("T{}:", i);

            for msg_num in 1..=5 {
                let msg = format!("{} message {}", prefix, msg_num);
                
                tx_clone.send(msg).unwrap();
            }
        });
    }

    drop(tx);

    for recieved in rx {
        println!("{}", recieved);
    }
}