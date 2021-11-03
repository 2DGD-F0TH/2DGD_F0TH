class Player{
    constructor(){
        this.coyote_time_started = False;
        this.coyote_time = 0;
        this.onground = False
        this.has_jumped = False
    }
    // ...
    update(dt){
        // ...
        if (this.onground){
            // Do stuff when player is on ground
            // ...
        }else{
            if (!this.has_jumped){
                // Player is not on ground and has not jumped, the player's falling
                if (!this.coyote_time_started){
                    this.coyote_time_started = true;
                    this.coyote_time = 5;
                }else{
                    this.coyote_time = this.coyote_time - dt;
                }
            }
        }
    }

    jump(){
        // This function takes care of jumping
        // ...
        if (this.coyote_time > 0){
            // Do Jump
        }
        // ...
    }
}
