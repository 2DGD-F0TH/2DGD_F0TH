// Operators can be treated as functions, that means you can
// Assign them to a variable.

// This...

if (a == b && c == d){
    // Do something...
}

// Is equivalent to this

let complex_condition = (a == b && c == d);

if (complex_condition){
    // ...
}

// ---------------8<---------------

// Also this...

function thing(a, b){
    if (a == b){
        return true;
    }else{
        return false;
    }
}

// Is equivalent to...

function thing(a, b){
    return a == b;
}
