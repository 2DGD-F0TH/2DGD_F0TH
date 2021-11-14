class Player{
    constructor(){
        this.speed = [0, 0];
        this.needs_update = false;
    }
    // ...
    input(){
        // ...
        if (right_key.is_Pressed){
            this.speed[0] = this.speed[0] + 1;  // Move right
            this.needs_update = true;
            // ...
        }
        if (up_key.is_Pressed){
            this.needs_update[1] = this.speed[1]  -100;  // Move up (jump)
            this.needs_update = true;
        }
        // ...
    }

    update(dt){
        if (this.needs_update){
            // Do Update instructions
            // ...
        }
    }
}
