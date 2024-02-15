// ...
class Player{
    constructor(){
        this.MAX_SPEED = 50.0;  // Maximum speed
        this.ACCEL = 15.0;  // The acceleration rate
        this.DECEL = 30.0;  // The deceleration rate
        this.input_accel = new Vector2();  // Defines the direction we are accelerating
        this.velocity = new Vector2();  // Defines the direction and magnitude of our speed
        this.position = new Vector2();  // Defines our current position, in (x,y) coordinates
        this.is_moving = False;  // Tells us if we're moving
    }

    handle_input(){
        // First of all, we need to zero the input_accel, or we'll be working on "residual data"
        input_accel = Vector2.ZERO;
        // Now we can handle movement
        if (KEYBOARD.Left_Arrow_Pressed){
            this.input_accel.x = this.input_accel.x - 1;
        }
        if (KEYBOARD.Right_Arrow_Pressed){
            this.input_accel.x = this.input_accel.x + 1;
        }
        if (KEYBOARD.Down_Arrow_Pressed){
            this.input_accel.y = this.input_accel.y + 1;
        }
        if (KEYBOARD.Right_Arrow_Pressed){
            this.input_accel.y = this.input_accel.y - 1;
        }
        // If any component of the acceleration vector is not zero, we are moving
        if (this.input_accel != Vector2.ZERO){
            this.is_moving = true;
        }
    }

    handle_movement(dt){
        if (this.is_moving){
            // Vectors will take care of summing forces
            this.velocity = this.velocity + ACCEL * dt * this.input_accel;
            // We need to clamp the speed, to avoid going too fast
            this.velocity.clamp(MAX_SPEED);
        }else{
            // We are stopping, let's subtract the deceleration
            let velocity_value = this.velocity.length() - DECEL * dt;
            if (velocity_value < 0){
                // If, After decelerating, we have a negative value, we need to make it zero or the object will start moving backwards
                velocity_value = 0;
            }
            // We are just changing the length of the vector, so we can just clamp its length
            this.velocity.clamp(velocity_value);
        }

        // Now it's time to move the object
        this.position = this.position + this.velocity;
    }
}
