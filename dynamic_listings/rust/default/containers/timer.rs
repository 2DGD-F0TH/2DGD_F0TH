struct Timer {
    time: f32,
    set_time: f32,
    function_to_execute: fn(),
    one_shot: bool,
    active: bool,
}

impl Timer {
    pub fn new(time_set: f32, funct: fn(), oneshot: Option<bool>, act: Option<bool>) -> Self {
        Self {
            // We prepare the timer and memorize the setting
            time: time_set,
            set_time: time_set,
            // The function pointer should already be prepared with the arguments
            function_to_execute: funct,
            // Is this timer one-shot then disable?
            one_shot: oneshot.unwrap_or_default(),
            // Does this timer need to be active when constructed?
            active: act.unwrap_or_default(),
        }
    }

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
            // We reset the timer (we may need to re-activate it manually later)
            self.time = self.set_time;
        }
    }
}
