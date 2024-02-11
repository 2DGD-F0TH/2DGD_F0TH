class FirstService{
    // Implementation here...
};

class SecondService{
    // Implementation here...
};

class Facade{
    /*
     * This class hides the complexities of using
     * FirstService and SecondService from the user
     * by "wrapping" them in a comfortable startAll
     * function
     */
    private:
        FirstService service1;
        SecondService service2;

    public:
        Facade(){
            service1 = FirstService();
            service2 = FirstService();
        }

        bool startAll(){
            /*
             * The facade starts all the services and does
             * some status checking, this is hidden from the
             * user.
             * Returns true if all services started successfully
             * false otherwise
             */
            bool firstServiceStarted = this.service1.start();
            if (!firstServiceStarted){
                return false;
            }
            bool secondServiceStarted = this.service2.start();
            if (!secondServiceStarted){
                return false;
            }
            // Here everything started successfully
            return true;
        }
};
