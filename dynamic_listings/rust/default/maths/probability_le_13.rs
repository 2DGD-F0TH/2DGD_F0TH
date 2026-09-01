fn main() {
    let mut happened = 0;
    // Monte Carlo Method we do 10000 "extractions"
    for _ in 0..10_000 {
        // Get a random number between 1 and 100
        let n = rand::random_range(1..=100);
        if n <= 13 {
            // If it's less or equal than 13, we have a match!
            happened += 1;
        }
    }
    // We print the result
    println!("{happened}");
}
