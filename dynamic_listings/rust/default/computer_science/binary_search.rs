fn binary_search(lst: &[u32], item: u32) -> Option<usize> {
    let mut first = 0;
    let mut last = lst.len() - 1;
    while first <= last {
        // Find the middle element
        let midpoint = (first + last) / 2;
        if lst[midpoint] == item {
            // We found it!
            return Some(midpoint);
        } else {
            if item < lst[midpoint] {
                // Continue on the "first half"
                last = midpoint - 1;
            } else {
                // Continue on the "second half"
                first = midpoint + 1;
            }
        }
    }
    // We return `None` to tell "not found"
    None
}
