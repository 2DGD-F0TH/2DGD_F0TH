function count_backwards(n){
    // Stop condition
    if (n == 0){
        // If we don't do this, we won't print 0
        console.log(n);
        return;
    }
    // Procedure
    console.log(n);
    // Recursive call
    count_backwards(n-1);
}
