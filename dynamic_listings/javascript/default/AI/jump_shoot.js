class Player{
    // ...
    registerShootingObserver(callback){
        // Function used to register an observer that will be called when the
        // player shoots a projectile.
    }
}

class JumpingBoss{
    // A boss that jumps when the player shoots. Sometimes.


    constructor(x, y, player){
        // ...
        this.player_shot = false;
        this.y_velocity = 0.0;
        this.on_ground = false;
        player.registerShootingObserver(this.setPlayerShot);
        // ...
    }

    setPlayerShot(){
        // Sets a state that tells the AI that the player shot a bullet
        this.player_shot = true;
    }

    jump(){
        // Sets the boss velocity to -10, making it jump
        if (this.on_ground){
            this.y_velocity = -10;
        }
    }

    update(dt){
        // ...
        if (this.player_shot == true){
            if (Math.floor(Math.random() * 5) + 1 == 1){
                // Jump only 20% of the times the player shoots
                this.jump();
            }
        }
        // We reset player_shot to false, if we didn't the boss would jump
        // a lot more often than 20% of the time
        this.player_shot = false;
        // ...
    }
}
