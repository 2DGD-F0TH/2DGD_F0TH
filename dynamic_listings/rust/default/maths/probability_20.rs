fn main() {
    let mut happened = 0;
    // Monte Carlo Method we do 10000 "extractions"
    for _ in 0..10_000 {
        // Get a random number between 1 and 5
        let n = rand::random_range(1..=5);
        if n == 1 {
            // If it's 1, we have a match!
            happened += 1;
        }
    }
    // We print the result
    println!("{happened}");
}
