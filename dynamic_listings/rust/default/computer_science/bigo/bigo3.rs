// -------------------------------------
// A is an array of integers
let n = a.len();
let mut swapped = false;

while swapped {
    swapped = false;
    for x in 0..n {
        if a[x - 1] > a[x] {
            let tmp = a[x - 1];
            a[x - 1] = a[x];
            a[x] = tmp;
            swapped = true;
        }
    }
}

// -------------------------------------

for item in a {
    println!("{item}");
}
