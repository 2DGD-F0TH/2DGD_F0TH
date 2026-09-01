// Operators can be treated as functions, that means you can
// Assign them to a variable.

// This...
if a == b && c == d {
    // Do something...
    return false;
}

// Is equivalent to this

let complex_condition = a == b && c == d;

if complex_condition {
    // ...
}

// ---------------8<---------------

// Also this...

fn thing(a: u32, b: u32) -> bool {
    if  a == b {
        true
    } else {
        false
    }
}

// Is equivalent to...

fn thing(a: u32, b: u32) -> bool {
    a == b
}
