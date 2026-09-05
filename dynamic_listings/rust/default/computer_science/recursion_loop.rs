# #![allow(dead_code)]
fn count_backwards(n: u32) {
    let mut n = n;
    // Condition for the loop
    while n != 0 {
        // The function body
        println!("{n}");
        // We update the condition to count down
        n = n - 1;
    }
}
