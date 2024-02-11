class FirstService{
    // Implementation here...
}

class SecondService{
    // Implementation here...
}

class Facade{
    /*
     * This class hides the complexities of using
     * FirstService and SecondService from the user
     * by "wrapping" them in a comfortable startAll
     * function
     */

    constructor(){
        this.service1 = new FirstService();
        this.service2 = new FirstService();
    }

    startAll(){
        /*
         * The facade starts all the services and does
         * some status checking, this is hidden from the
         * user.
         * Returns true if all services started successfully
         * false otherwise
         */
        let firstServiceStarted = this.service1.start();
        if (!firstServiceStarted){
            return false;
        }
        let secondServiceStarted = this.service2.start();
        if (!secondServiceStarted){
            return false;
        }
        // Here everything started successfully
        return true;
    }
}
