class Player{
    constructor(){
        this.JUMP_VELOCITY = -12.0;
        this.y_speed = 0;
    }
    // ...
    onJumpKeyPressed(){
        /* The jump key has just been pressed (doesn't account the jump key being
        pressed from previous frames) */
        this.y_speed = JUMP_VELOCITY;
    }

    onJumpKeyReleased(){
        // The jump key was just released, cut the y_speed so the jump is lower
        if (this.y_speed < JUMP_VELOCITY / 2){
            // The speed is higher than the cutoff speed (in absolute value)
            this.y_speed = JUMP_VELOCITY / 2;
        }
    }
}
