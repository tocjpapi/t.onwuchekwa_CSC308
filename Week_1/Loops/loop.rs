fn main(){
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            println!("count {}", count);
            break;
        }
    } 
}