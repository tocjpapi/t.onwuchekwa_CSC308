// Number 2

fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10,11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    // Closure that checks if a number is even
    let even_check = |n: &i32| n % 2 == 0;

    // Use the closure to filter
    let evens: Vec<i32> = vector.iter().cloned().filter(even_check).collect();

    println!("Even numbers: {:?}", evens);
}
