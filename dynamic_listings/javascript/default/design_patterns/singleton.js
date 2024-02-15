class Singleton {
    // This is actually a lazy implementation of the singleton pattern
    // since we can't instance classes at "import time"

    // This way the constructor always returns the singleton instance
    constructor(){
        if (!Singleton.INSTANCE){
            Singleton.INSTANCE = this;
        }
        return Singleton.INSTANCE;
    }

    static getInstance(){
        return Singleton.INSTANCE;
    }
}
