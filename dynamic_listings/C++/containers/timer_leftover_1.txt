class Timer{
    /*
     * ...
     * This is the same as the older version
     * ...
     */

    public:
        void update(float dt){
            if (!active){
                // We return directly if the timer is disabled
                return;
            }
            // Like any other entity, we update it
            time = time - dt;
            // When the timer "ticks", we execute the function
            if (time <= 0){
                function_to_execute();
                if (one_shot){
                    // If this timer is a one-shot, we disable it
                    active = false;
                }
                // We reset the timer differently, by adding the "set time" until we have a positive value
                while (time <= 0){
                    time = time + set_time;
                }
            }
        }
};
