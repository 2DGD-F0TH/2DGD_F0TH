function main(){
    let happened = 0;
    // Monte Carlo Method we do 10000 "extractions"
    for (let i = 0; i < 1000; i++){
        // Get a random number between 1 and 100
        let n = Math.random() * 100 + 1;
        if (n <= 13){
            // If it's less or equal than 13, we got a match
            happened++;
        }
    }
    // We print the result
    console.log(happened / 10000)
}
