use std::io;

fn first(){
    println!("Enter the word you want to find the last word of using slices:");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");
    let sentence = sentence.trim();
    if let Some(pos) = sentence.rfind(' ') {
        let last_word = &sentence[pos + 1..];
        println!("The last word is: {}", last_word);
    } else if !sentence.is_empty() {
        println!("The last word is: {}", sentence);
    } else {
        println!("The sentence is empty.");
    }
}