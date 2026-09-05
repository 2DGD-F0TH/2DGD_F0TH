# #![allow(dead_code)]
fn bubble_sort(a: &mut [u32]) {
    let n = a.len();
    // Traverse Through all Elements
    for x in 0..n {
        // Last x elements are in place due to the nature of the sort
        for y in 0..(n - x - 1) {
            // Swap if the element found is greater than the next element
            if a[y] > a[y + 1] {
                let tmp = a[y];
                a[y] = a[y + 1];
                a[y + 1] = tmp;
            }
        }
    }
}
