fn count_backwards(n: u32) {
    // Stop condition
    if n == 0 {
        // If we don't do this, we won't print 0
        println!("{n}");
        return;
    }
    // Procedure
    println!("{n}");
    // Recursive call
    count_backwards(n - 1);
}
