class Command{
    // This is the abstract class that will be used as interface
    constructor(){
        // In javascript there are no real facilities for abstract classes
        // So we need to play around
        if (new.target == Handler){
            // The new.target changes in derivative classes
            throw new TypeError("Cannot construct abstract objects")
        }
    }

    execute(){};
}

class JumpCommand extends Command{
    // This will implement the execute method
    execute(){
        jump();
    }

    jump(){
        // DO STUFF
    }
}
