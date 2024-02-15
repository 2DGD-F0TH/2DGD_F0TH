class FSM{
    /*
     * This class defines a Finite State Machine
     * The currently active state is represented by a function
     * pointer
     */
    constructor(){
        this.current_state = null;
    }

    setState(f){
        /*
         * Sets the state, from this point on, update will
         * change its strategy
         */
        this.current_state = f;
    }

    update(dt){
        // If there is a current state, execute it
        if (this.current_state != null){
            this.current_state(dt);
        }
    }
}
