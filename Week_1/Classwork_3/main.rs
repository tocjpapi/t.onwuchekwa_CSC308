fn main(){
    let mut usage = String::new();
    println!("Enter the total electricity usage in kWh:");
    std::io::stdin().read_line(&mut usage).expect("Failed to read line");
    let usage: f64 = usage.trim().parse().expect("Please enter a valid number");
    let bill = if usage > 100.0 {
        usage * 25.0
    } else if usage > 200.0 {
        usage * 30.0
    } else {
        usage * 20.0
    };
    println!("Total electricity bill: {:.2}", bill);
}