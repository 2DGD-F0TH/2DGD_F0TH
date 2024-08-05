#include <functional>

class Timer{
    /*
     * This is a simple timer class that executes a function after
     * a certain amount of time
     */
    private:
        float time;
        float set_time;
        std::function<void()> function_to_execute;
        bool one_shot;
        bool active;

    public:
        Timer(float time_set, function_ptr funct, bool oneshot = false, bool act=false){
            // We prepare the timer and memorize the setting
            time = time_set;
            set_time = time;
            // The function pointer should already be prepared with the arguments
            function_to_execute = funct;
            // Is this timer one-shot then disable?
            one_shot = oneshot;
            // Does this timer need to be active when constructed?
            active = act;
        }

        void update(float dt){
            if (!active){
                // We return directly if the timer is disabled
                return;
            }
            // Like any other entity, we update it
            time -= dt;
            // When the timer "ticks", we execute the function
            if (time <= 0){
                function_to_execute();
                if (one_shot){
                    // If this timer is a one-shot, we disable it
                    active = false;
                }
                // We reset the timer (we may need to re-activate it manually later)
                time = set_time;
            }
        }
};
