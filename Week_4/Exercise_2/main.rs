use std::io;

struct Circle {
    radius: f64,
}

fn main() {
    println!("Please enter the radius of the circle:");

    let mut radius_input = String::new();
    io::stdin()
        .read_line(&mut radius_input)
        .expect("Failed to read line");

    let radius: f64 = match radius_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let circle = Circle { radius };

    println!("To find area press A, to find circumference press C:");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim().to_uppercase();

    let area = 3.14 * circle.radius * circle.radius;
    let circumference = 2.0 * 3.14 * circle.radius;

    if choice == "A" {
        println!("The area of the circle is: {:.2}", area);
    } else if choice == "C" {
        println!("The circumference of the circle is: {:.2}", circumference);
    } else {
        println!("Invalid choice. Please enter 'A' or 'C'.");
    }
}
