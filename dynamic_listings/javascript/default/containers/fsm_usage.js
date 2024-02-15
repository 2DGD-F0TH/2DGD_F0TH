class Enemy{
    /*
     * Represents a simple enemy
     */
    // ...

    constructor(x, y){
        this.position_x = x;
        this.position_y = y;
        PURSUETIME = 10.0;
        pursue_timer = new Timer();
        brain = new FSM();
        this.brain.setState(this.patrol);
    }

    sees(other){
        /*
         * Checks if the enemy sees an entity
         */
        // ...
    }

    patrol(dt){
        // Normal patrolling of the enemy
        // Move, turn, path find...
        if (this.sees(player)){
            // ...
            // Pursue for xx seconds
            this.pursue_timer.set(this.PURSUETIME);
            this.pursue_timer.start();
            // Change FSM State
            this.brain.setState(this.pursue);
        }
    }

    pursue(dt){
        // Tries to pursue the enemy
        if (this.sees(player)){
            // Continue Pursuing, by resetting the timer
            this.pursue_timer.set(this.PURSUETIME);
            // ...
        }
        // ...
        // If the enemy is not in sight for xx seconds
        if (this.pursue_timer.is_finished()){
            // go back to patrolling
            this.brain.setState(this.patrol);
        }
    }

    update(dt){
        // The enemy update function
        // ...
        this.pursue_timer.update(dt);
        this.brain.update(dt);
        // ...
    }
}
