#[derive(Clone, Default)]
struct Fsm {
    /*
     * This class defines a Finite State Machine
     * The currently active state is represented by a function
     * pointer
     */
    current_state: Option<fn(&mut Enemy, f32)>,
}

impl Fsm {
    fn set_state(&mut self, f: fn(&mut Enemy, f32)) {
        /*
         * Sets the state, from this point on, update will
         * change its strategy
         */
        self.current_state = Some(f);
    }

    fn update(&mut self, enemy: &mut Enemy, dt: f32) {
        if let Some(current_state) = self.current_state {
            current_state(enemy, dt);
        }
    }
}
