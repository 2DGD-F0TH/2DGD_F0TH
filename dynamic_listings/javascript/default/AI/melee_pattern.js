class Boss{
    constructor(player){
        this.player = player;
        this.player_too_far = false;
        this.base_movement_velocity = 10;
        this.too_far_space = 30;
        this.velocity = Vector2D();
        this.position = Vector2D();
    }

    update(dt){
        // ...
        if (Math.abs(player.position.x - this.position.x) > this.too_far_space){
            if (Math.abs(player.position.y - this.position.y) > this.too_far_space){
                // The player is too close
                if (Math.random(1,6) == 1){
                    // Add a bit of randomization
                    this.player_too_far = true;
                }
            }
        }
        // We're using a variable to preserve the "too far" state between frames
        if (this.player_too_far){
            // The player is too far, close in
            let distance = player.position - this.position;
            // Make it a direction
            let direction = distance.normalize();
            // This is the direction the boss should go now, transfer it to velocity
            this.velocity = direction * this.base_movement_velocity;
        }
        // ...
        // The boss and player now have moved, let's see if they're close enough
        // ...
        if (Math.abs(player.position.x - this.position.x) < this.too_far_space){
            if (Math.abs(player.position.y - this.position.y) < this.too_far_space){
                // The player is close enough now
                this.player_too_far = false;
            }
        }
    }
}
