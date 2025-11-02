use std::io;


struct Student {
    name: String,
    score: f32,
}


impl Student {

    fn new(name: String, score: f32) -> Student {
        Student { name, score }
    }
    

    fn has_passed(&self) -> bool {
        self.score >= 50.0
    }
    

    fn get_status(&self) -> &str {
        if self.has_passed() {
            "PASSED"
        } else {
            "FAILED"
        }
    }
    

    fn display_evaluation(&self) {
        println!("\n--- Course Evaluation ---");
        println!("Student Name: {}", self.name);
        println!("Score: {:.2}", self.score);
        println!("Status: {}", self.get_status());
        
        if self.has_passed() {
            println!("Congratulations! You passed the course.");
        } else {
            println!("Unfortunately, you did not pass. Keep trying!");
        }
    }
}

fn main() {

    println!("=== Student Course Evaluation System ===\n");
    println!("Enter student's name:");
    

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim().to_string();
    

    println!("Enter student's score (0-100):");
    

    let mut score_input = String::new();
    io::stdin()
        .read_line(&mut score_input)
        .expect("Failed to read score");
    

    let score: f32 = match score_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid score entered. Please enter a number.");
            return;
        }
    };
    

    if score < 0.0 || score > 100.0 {
        println!("Score must be between 0 and 100.");
        return;
    }
    

    let student = Student::new(name, score);
    

    student.display_evaluation();
}