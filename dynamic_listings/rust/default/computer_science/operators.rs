# #![allow(dead_code)]
# fn foo() -> bool {
# let a = true;
# let b = true;
# let c = true;
# let d = true;
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
# true
# }

// ---------------8<---------------

// Also this...
# mod mod1 {
fn thing(a: u32, b: u32) -> bool {
    if  a == b {
        true
    } else {
        false
    }
}
# }

# mod mod2 {
// Is equivalent to...

fn thing(a: u32, b: u32) -> bool {
    a == b
}
# }
