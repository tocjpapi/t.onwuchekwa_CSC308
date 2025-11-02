use std::io;

struct BankAccount{
    balance: f32,
}

fn main(){
    println!("Enter initial balance:");
    let mut balance_input = String::new();

    io::stdin().read_line(&mut balance_input).expect("Failed to read line");

    let balance = match balance_input.trim().parse()
}