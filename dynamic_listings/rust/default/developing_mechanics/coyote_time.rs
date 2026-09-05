# #![allow(dead_code)]
struct Player {
    coyote_time_started: bool,
    coyote_time: f32,
    onground: bool,
    has_jumped: bool,
}

impl Player {
    // ...
    fn update(&mut self, dt: f32) {
        // ...
        if self.onground {
            // Do stuff when player is on ground
            // ...
        } else {
            if !self.has_jumped {
                // Player is not on the ground and has not jumped, the player is falling
                if !self.coyote_time_started {
                    self.coyote_time_started = true;
                    self.coyote_time = 5.;
                } else {
                    self.coyote_time -= dt;
                }
            }
        }
    }

    fn jump(&mut self) {
        // This function takes care of jumping
        // ...
        if self.coyote_time > 0. {
            // Do Jump
            // ...
        }
    }
}
