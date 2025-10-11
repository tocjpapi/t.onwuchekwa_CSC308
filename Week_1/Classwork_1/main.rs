use std::io;
fn main(){
    println!("Temperature Converter");

    loop {
    let mut choice = String::new();
    println!("Do you want convert from Fahrenheit to Celsius (F) or from Celsius to Fahrenheit (C) or end it (E)?");
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().to_uppercase();

    if choice == "F" {
        fahrenheit_to_celsius();
    }

    else if choice == "C" {
        celsius_to_fahrenheit();
    }

    else if choice == "E" {
        println!("Exiting the program. Goodbye!");
        break;
    }
    else {
        println!("Invalid choice. Please enter 'F' or 'C'.");
    }

    }
}


fn fahrenheit_to_celsius() {
    let mut fahrenheit = String::new();
    println!("Please Enter temperature in Fahrenheit:");
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
    let fahrenheit: f64 = match fahrenheit.trim().parse()
    {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("Temperature: {:.2}℉", fahrenheit);
    println!("Converted: {:.2}℃", celsius);
}

fn celsius_to_fahrenheit() {
    let mut celsius = String::new();
    println!("Please Enter temperature in Celsius:");
    std::io::stdin().read_line(&mut celsius).expect("Failed to read line");
    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("Temperature: {:.2}℃", celsius);
    println!("Converted: {:.2}℉", fahrenheit);
}