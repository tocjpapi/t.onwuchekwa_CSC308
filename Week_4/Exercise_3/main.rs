use std::io;

struct BankAccount{
    balance: f32,
}

fn main(){
    println!("Enter initial balance:");
    let mut balance_input = String::new();

    io::stdin().read_line(&mut balance_input).expect("Failed to read line");

    let balance = match balance_input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please Enter a valid number.");
            return;
        }
    };

    let account = BankAccount{balance};

    println!("To check balance press 1, to deposit press 2, To withdraw press 3");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();
    if choice == "1"{
        println!("Your balance is: {:.2}", account.balance);
    }
    else if choice == "2"{
        println!("Enter amount to deposit:");
        let mut deposit_input = String::new();
        io::stdin().read_line(&mut deposit_input).expect("Failed to read line");
        let deposit_amount: f32 = match deposit_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };
        let new_balance = account.balance + deposit_amount;
        println!("New balance after deposit is: {:.2}", new_balance);
    }
    else if choice == "3"{
        println!("Enter amount to withdraw:");
        let mut withdraw_input = String::new();
        io::stdin().read_line(&mut withdraw_input).expect("Failed to read line");
        let withdraw_amount: f32 = match withdraw_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };
        if withdraw_amount > account.balance{
            println!("Insufficient funds. Your balance is: {:.2}", account.balance);
        }
        else{
            let new_balance = account.balance - withdraw_amount;
            println!("New balance after withdrawal is: {:.2}", new_balance);
        }
    }
    else{
        println!("Invalid choice. Please enter 1, 2, or 3.");
    }
}