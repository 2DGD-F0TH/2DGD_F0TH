class LazySingleton {

    // This way the constructor always returns the singleton instance
    constructor(){
        // Multi-threading: manage race conditions
        // ----- Critical region start -----
        if (!Singleton.INSTANCE){
            Singleton.INSTANCE = this;
        }
        // ----- Critical region end -----
        return Singleton.INSTANCE;
    }

    static getInstance(){
        return Singleton.INSTANCE;
    }
}
