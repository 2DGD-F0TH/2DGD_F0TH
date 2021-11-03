class Timer{
    /*
     * This is a simple timer class that executes a function after
     * a certain amount of time
     */
    constructor(){
        this.time = 0;
        this.set_time;
        function_to_execute = function(){};
        this.one_shot = false;
        this.active = true;
    }

    constructor(time, func, one_shot, active){
        // We prepare the timer and memorize the setting
        this.time = time;
        this.set_time = time;
        // The function pointer should already be prepared with the arguments
        this.function_to_execute = func;
        // Is this timer one-shot then disable?
        this.one_shot = one_shot;
        if (one_shot == undefined){
            this.one_shot = false;
        }
        // Does this timer need to be active when constructed?
        this.active = active;
        if (active == undefined){
            this.active = false;
        }
    }

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
            // We reset the timer (we may need to re-activate it manually later)
            this.time = this.set_time;
        }
    }
}
