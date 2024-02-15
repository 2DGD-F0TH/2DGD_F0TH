class Boss{
    constructor(player){
        this.player = player;
        this.player_too_close = false;
        this.base_movement_velocity = 10;
        this.too_close_space = 20;
        this.velocity = Vector2D();
        this.position = Vector2D();
    }

    update(dt){
        // ...
        if (Math.abs(player.position.x - this.position.x) < this.too_close_space){
            if (Math.abs(player.position.y - this.position.y) < this.too_close_space){
                // The player is too close
                if (Math.random(1,6) == 1){
                    // Add a bit of randomization
                    this.player_too_close = true;
                }
            }
        }
        // We're using a variable to preserve the "too close" state between frames
        if (this.player_too_close){
            // The player is too close, make some distance
            let distance = this.position - player.position;
            // Make it a direction
            let direction = distance.normalize();
            // This is the direction the boss should go now, transfer it to velocity
            this.velocity = direction * this.base_movement_velocity;
        }
        // ...
        // The boss and player now have moved, let's see if they're far enough
        // ...
        if (Math.abs(player.position.x - this.position.x) > this.too_close_space){
            if (Math.abs(player.position.y - this.position.y) > this.too_close_space){
                // The player is far enough now
                this.player_too_close = false;
            }
        }
    }
}
