// ...
struct Player {
    input_accel: Vector2D,
    velocity: Vector2D,
    position: Vector2D,
    is_moving: bool,
}

impl Player {
    const MAX_SPEED: f32 = 50.;
    const ACCEL: f32 = 15.;
    const DECEL: f32 = 30.;

    pub fn handle_input(&mut self) {
        // First of all, we need to zero the input_accel, or we'll be working on "residual data"
        self.input_accel = Vector2D::default();
        // Now we can handle movement
        if KEYBOARD.left_arrow_pressed {
            self.input_accel.x -= 1.;
        }
        if KEYBOARD.right_arrow_pressed {
            self.input_accel.x += 1.;
        }
        if KEYBOARD.down_arrow_pressed {
            self.input_accel.y += 1.;
        }
        if KEYBOARD.up_arrow_pressed {
            self.input_accel.y -= 1.;
        }
        // If any component of the acceleration vector is not zero, we are moving
        if self.input_accel != Vector2D::default() {
            self.is_moving = true;
        }
    }

    pub fn handle_movement(&mut self, dt: f32) {
        if self.is_moving {
            // Vectors will take care of summing forces
            self.velocity += self.input_accel * Self::ACCEL * dt;
            // We need to clamp the speed, to avoid going too fast
            self.velocity.max_each(Self::MAX_SPEED);
        } else {
            // We are stopping, let's subtract the deceleration
            let mut velocity_value = self.velocity.length() - Self::DECEL * dt;
            if velocity_value < 0. {
                // If, After decelerating, we have a negative value, we need to make it zero or the object will start moving backwards
                velocity_value = 0.;
            }
            // We are just changing the length of the vector, so we can just clamp its length
            self.velocity.max_each(velocity_value);
        }
        // Now it's time to move the object
        self.position += self.velocity;
    }
}
