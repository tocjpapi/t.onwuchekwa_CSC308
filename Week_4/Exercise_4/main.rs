use std::io;

fn main() {

    println!("Enter a sentence from a student essay:");
    

    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    

    let sentence = sentence.trim();
    analyze_sentence(sentence);
}

fn analyze_sentence(sentence: &str) {

    let words: Vec<&str> = sentence
        .split_whitespace()
        .collect();
    

    if words.is_empty() {
        println!("No words found in the sentence.");
        return;
    }
    

    let longest = words
        .iter()
        .max_by_key(|word| word.len())
        .unwrap();
    

    let shortest = words
        .iter()
        .min_by_key(|word| word.len())
        .unwrap();
    

    println!("\n--- Analysis Results ---");
    println!("Longest word: \"{}\" ({} characters)", longest, longest.len());
    println!("Shortest word: \"{}\" ({} characters)", shortest, shortest.len());
}