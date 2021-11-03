class Timer{
    /*
     * ...
     * This is the same as the older version
     * ...
     */

    update(dt){
        if (!this.active){
            // We return directly if the timer is disabled
            return;
        }
        // Like any other entity, we update it
        this.time = this.time - dt;
        // When the timer "ticks", we execute the function
        if (this.time <= 0){
            this.function_to_execute();
            if (this.one_shot){
                // If this timer is a one-shot, we disable it
                this.active = False;
            }
            // We reset the timer differently, by adding the "set time" with a multiplier
            // this.time is guaranteed to be negative or zero, by dividing by a negative number
            // we have a positive multiplier
            let multiplier = ceil(this.time / -this.set_time);
            this.time = this.time + (multiplier * this.set_time);
        }
    }
}
