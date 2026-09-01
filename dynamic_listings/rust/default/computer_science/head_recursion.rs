fn count_forwards(n: u32) {
    // Stop condition
    if n == 0 {
        // If we don't do this, we won't print 0
        println!("{n}");
        return;
    }
    // Recursive call
    count_forwards(n - 1);
    // Procedure
    println!("{n}");
}
