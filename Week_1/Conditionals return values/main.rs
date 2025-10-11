fn main(){
    let score = 85;
    let grade = if score >= 90 {"Excellent"} else if score >= 75 {"Great"} else {"You can do better"};

    println!("You're doing: {}", grade);
}