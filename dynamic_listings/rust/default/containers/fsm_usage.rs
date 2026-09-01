struct Enemy {
    /*
     * Represents a simple enemy
     */
    position_x: f32,
    position_y: f32,
    pursue_timer: Timer,
    brain: Fsm,
}

impl Enemy {
    const PURSUETIME: f32 = 10.;

    fn new(x: f32, y: f32) -> Self {
        /*
         * Constructor
         */
        let mut enemy = Self {
            position_x: x,
            position_y: y,
            pursue_timer: Timer::default(),
            brain: Fsm::default(),
        };

        enemy.brain.set_state(Self::patrol);

        enemy
    }

    fn sees(&self, other: &Entity) -> bool {
        /*
         *Implements logic for the "sight" of the enemy
         */
        // ...
    }

    fn patrol(&mut self, dt: f32) {
        // Normal patrolling of the enemy
        // Move, turn, path find...
        if self.sees(&player) {
            // ...
            // Pursue for xx seconds
            self.pursue_timer.set(Self::PURSUETIME);
            self.pursue_timer.start();
            // Change FSM State
            self.brain.set_state(Self::pursue);
        }
    }

    fn pursue(&mut self, dt: f32) {
        // Tries to pursue the enemy
        if self.sees(&player) {
            // Continue Pursuing, by resetting the timer
            self.pursue_timer.set(Self::PURSUETIME);
            // ...
        }
        // ...
        // If the enemy is not in sight for xx seconds
        if self.pursue_timer.is_finished() {
            // go back to patrolling
            self.brain.set_state(Self::patrol);
        }
    }

    fn update(&mut self, dt: f32) {
        // The enemy update function
        // ...
        self.pursue_timer.update(dt);
        let mut brain = self.brain.clone();
        brain.update(self, dt);
        self.brain = brain;
        // ...
    }
}
