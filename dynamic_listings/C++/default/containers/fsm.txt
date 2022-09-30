#include <functional>

class FSM{
    /*
     * This class defines a Finite State Machine
     * The currently active state is represented by a function
     * pointer
     */

    std::function<void(float)> current_state = nullptr;

    void setState(std::function<void(float)> f){
        /*
         * Sets the state, from this point on, update will
         * change its strategy
         */
        current_state = f;
    }

    void update(float dt){
        // If there is a current state, execute it
        if (current_state != nullptr){
            current_state(dt);
        }
    }
};
