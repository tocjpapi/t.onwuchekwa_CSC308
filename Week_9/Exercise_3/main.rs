use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        println!("\n----- Notes Menu -----");
        println!("1. Add a note");
        println!("2. View all notes");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_note(),
            "2" => view_notes(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn add_note() {
    print!("Enter your note: ");
    io::stdout().flush().unwrap();

    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();

    let timestamp = current_timestamp();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")
        .expect("Failed to open notes file");

    writeln!(file, "[{}] {}", timestamp, note.trim())
        .expect("Failed to write note");

    println!("Note saved!");
}

fn view_notes() {
    let mut file = match File::open("notes.txt") {
        Ok(f) => f,
        Err(_) => {
            println!("No notes found yet.");
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("\n----- All Notes -----");
    if contents.trim().is_empty() {
        println!("(No notes yet)");
    } else {
        println!("{contents}");
    }
}

fn current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Simple readable timestamp: seconds since epoch
    // (You can replace this with chrono if you want fancy times)
    format!("Time: {now}")
}
