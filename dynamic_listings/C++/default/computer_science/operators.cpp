// Operators can be treated as functions, that means you can
// Assign them to a variable.

// This...
if (a == b && c == d){
    // Do something...
    return false;
}

// Is equivalent to this

bool complex_condition = (a == b && c == d);

if (complex_condition){
    // ...
}

// ---------------8<---------------

// Also this...

bool thing(int a, int b){
    if (a == b){
        return true;
    }else{
        return false;
    }
}

// Is equivalent to...

bool thing(int a, int b){
    return a == b;
}
