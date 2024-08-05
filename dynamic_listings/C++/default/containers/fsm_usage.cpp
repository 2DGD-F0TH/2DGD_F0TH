class Enemy{
    /*
     * Represents a simple enemy
     */
    // ...
    const float PURSUETIME = 10.0;
    float position_x = 0.0;
    float position_y = 0.0;
    Timer pursue_timer = Timer();
    FSM brain = FSM();

    Enemy(float x, float y){
        /*
         * Constructor
         */
        position_x = x;
        position_y = y;
        brain.setState(patrol);
    }

    bool sees(Entity other){
        /*
         *Implements logic for the "sight" of the enemy
         */
        // ...
    }

    void patrol(float dt){
        // Normal patrolling of the enemy
        // Move, turn, path find...
        if (sees(player)){
            // ...
            // Pursue for xx seconds
            pursue_timer.set(PURSUETIME);
            pursue_timer.start();
            // Change FSM State
            brain.setState(pursue);
        }
    }

    void pursue(float dt){
        // Tries to pursue the enemy
        if (sees(player)){
            // Continue Pursuing, by resetting the timer
            pursue_timer.set(PURSUETIME);
            // ...
        }
        // ...
        // If the enemy is not in sight for xx seconds
        if (pursue_timer.is_finished()){
            // go back to patrolling
            brain.setState(patrol);
        }
    }

    void update(float dt){
        // The enemy update function
        // ...
        pursue_timer.update(dt)
        brain.update(dt)
        // ...
    }
};
