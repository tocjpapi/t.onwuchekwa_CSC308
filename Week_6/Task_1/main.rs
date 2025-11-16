//Number 1

fn main(){
    let factorial = |n: u64| -> u64 {
    if n <= 1 {
        1
    }

    else{
        (1..=n).product()
    }
};

   println!("{}",factorial(4));
}

