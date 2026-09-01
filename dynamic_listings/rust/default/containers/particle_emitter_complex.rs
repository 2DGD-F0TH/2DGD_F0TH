struct Emitter {
    /*
     * This is a simple particle emitter, it contains a list
     * of particles and it updates and manages them
     */
    origin: Vector2D,
    particles: Vec<Particle>,
    // Defines if this emitter streams continuously or only a burst of particles
    one_shot: bool,
}

impl Emitter {
    pub fn new(origin: Vector2D, one_shot: bool) -> Self {
        Self {
            origin,
            particles: vec![Particle::new(); 8], // We prepare 8 particles
            one_shot,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update the entire system, by updating each particle
        for particle in &mut self.particles {
            if self.one_shot {
                if particle.is_dead() {
                    continue;
                } else {
                    particle.update(dt);
                }
            } else {
                if particle.is_dead() {
                    particle.reset(); // Resets the state of the particle
                    particle.set_position(self.origin);
                }
                particle.update(dt);
            }
        }
    }
}
