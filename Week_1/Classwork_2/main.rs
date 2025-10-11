fn main(){
    let mut bill = String::new();
    println!("Enter the total bill amount:");
    std::io::stdin().read_line(&mut bill).expect("Failed to read line");
    let bill: f64 = bill.trim().parse().expect("Please enter a valid number");

    if bill > 5000.0 {
        let discount = bill * 0.10;
        let total = bill - discount;
        println!("You get a 10% discount! Your total bill is: {:.2}", total);
    }

    else if bill > 10000.0{
        let discount = bill * 0.15;
        let total = bill - discount;
        println!("You get a 15% discount! Your total bill is: {:.2}", total);  
    }
    
     else {
        println!("No discount applied. Your total bill is: {:.2}", bill);
    }
}