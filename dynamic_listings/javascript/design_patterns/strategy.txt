class Strategy{
    // This class defines the strategy interface the client will refer to

    constructor(){
        // In javascript there are no real facilities for abstract classes
        // So we need to play around
        if (new.target == Handler){
            // The new.target changes in derivative classes
            throw new TypeError("Cannot construct abstract objects")
        }
    }

    algorithm(){
        // This algorithm will be implemented by the subclasses
    }
}

class ConcreteStrategy1 extends Strategy{
    algorithm(){
        // Real implementation of the algorithm
        // DO STUFF
    }
}

class ConcreteStrategy2 extends Strategy{
    algorithm(){
        // Real implementation of the algorithm
        // DO STUFF SLIGHTLY DIFFERENTLY
    }
}

// Example Usage
function main(){
    let to_execute = null;
    if (condition){
        to_execute = new ConcreteStrategy1();
    }else{
        to_execute = new ConcreteStrategy2();
    }
    to_execute.algorithm();  // This will execute 1 or 2 depending on "condition"
}


