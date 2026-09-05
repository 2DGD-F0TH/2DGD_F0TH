# #![allow(dead_code)]
struct Timer {
    /*
     * ...
     * This is the same as the older version
     * ...
     */
#    time: f32,
#    set_time: f32,
#    function_to_execute: fn(),
#    one_shot: bool,
#    active: bool,
}

impl Timer {
    pub fn update(&mut self, dt: f32) {
        if !self.active {
            // We return directly if the timer is disabled
            return;
        }
        // Like any other entity, we update it
        self.time -= dt;
        // When the timer "ticks", we execute the function
        if self.time <= 0. {
            (self.function_to_execute)();
            if self.one_shot {
                // If this timer is a one-shot, we disable it
                self.active = false;
            }
            // We reset the timer differently, by adding the "set time"
            self.time += self.set_time;
        }
    }
}
