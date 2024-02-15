function main(){
    let happened = 0;
    // Monte Carlo Method we do 10000 "extractions"
    for (let i = 0; i < 1000; i++){
        // Get a random number between 1 and 5
        let n = Math.random() * 5 + 1;
        if (n == 1){
            // If it's 1, we have a match!
            happened++;
        }
    }
    // We print the result
    console.log(happened / 10000)
}
