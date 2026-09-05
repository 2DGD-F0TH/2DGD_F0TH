# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
# #[derive(Clone, Default)]
# struct Particle;
# impl Particle {
#     fn is_dead(&self) -> bool { todo!() }
#     fn update(&mut self, _: f32) {}
# }
struct Emitter {
    /*
     * This is a simple particle emitter, it contains a list
     * of particles and it updates and manages them
     */
    origin: Vector2D,
    particles: Vec<Particle>,
}

impl Emitter {
    pub fn new(origin: Vector2D) -> Self {
        Self {
            origin,
            particles: vec![Particle::default(); 8], // We prepare 8 particles
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update the entire system, by updating each particle
        for particle in &mut self.particles {
            if !particle.is_dead() {
                particle.update(dt);
            }
        }
    }
}
