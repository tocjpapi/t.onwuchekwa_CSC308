use std::io;

fn first(){
    println!("Enter the sentence you want to find the last word of:");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");
    let words: Vec<&str> = sentence.trim().split_whitespace().collect();
    if let Some(last_word) = words.last() {
        println!("The last word is: {}", last_word);
    } else {
        println!("The sentence is empty."); 
    }
}