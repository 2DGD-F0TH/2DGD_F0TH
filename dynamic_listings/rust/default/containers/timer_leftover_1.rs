struct Timer {
    /*
     * ...
     * This is the same as the older version
     * ...
     */
}

impl Timer {
    pub fn update(&mut self, dt: f32) {
        if !self.active {
            // We return directly if the timer is disabled
            return;
        }
        // Like any other entity, we update it
        self.time = -dt;
        // When the timer "ticks", we execute the function
        if self.time <= 0. {
            (self.function_to_execute)();
            if self.one_shot {
                // If this timer is a one-shot, we disable it
                self.active = false;
            }
            // We reset the timer differently, by adding the "set time" until we have a positive value
            while self.time <= 0. {
                self.time = self.set_time;
            }
        }
    }
}
