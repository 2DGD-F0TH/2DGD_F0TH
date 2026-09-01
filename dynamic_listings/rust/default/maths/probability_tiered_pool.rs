fn tiered_drop() -> u8 {
    // 1 = Common, 2 = Uncommon, 3 = Rare, 4 = Epic
    let n = rand::random_range(1..=100);
    match n {
        ..=50 => 1,
        // Since n <=50 has already returned false, we know this
        // branch will only happen if 50<n<=80
        ..=80 => 2,
        // Since both n<=50 and n<=80 both returned false, we know
        // this branch will only happen if 80<n<=95
        ..=95 => 3,
        // All other branches failed, so we'll get here only if
        // 95<n<=100
        _ => 4,
    }
}
