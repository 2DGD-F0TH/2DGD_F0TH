# #![allow(dead_code)]
struct Player {
    y_speed: f32,
}

impl Player {
    const JUMP_VELOCITY: f32 = -12.0;
    // ...
    fn on_jump_key_pressed(&mut self) {
        /* The jump key has just been pressed (doesn't account the jump key being
        pressed from previous frames) */
        self.y_speed = Self::JUMP_VELOCITY;
    }
    fn on_jump_key_released(&mut self) {
        // The jump key was just released, cut the y_speed so the jump is lower
        if self.y_speed < Self::JUMP_VELOCITY / 2. {
            // The speed is higher than the cutoff speed (in absolute value)
            self.y_speed = Self::JUMP_VELOCITY / 2.;
        }
    }
}
