# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
# #[derive(Clone)]
# struct Texture;
#[derive(Clone)]
struct Particle {
    /*
     * This is a simple particle class, it contains a reference to
     * its texture, as well as some state
     */
    texture: Texture,
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,
    lifespan: f32,
}

impl Particle {
    pub fn new(
        texture: Texture,
        position: Vector2D,
        velocity: Vector2D,
        acceleration: Vector2D,
        lifespan: Option<f32>,
    ) -> Self {
        Self {
            // We prepare the particle for usage
            texture,
            position,
            velocity,
            acceleration,
            lifespan: lifespan.unwrap_or(2_000.), // About 2 seconds by default
        }
    }

    pub fn update(&mut self, dt: f32) {
        // We update the velocity (assuming dt is in milliseconds)
        self.velocity += self.acceleration;
        // Then the position
        self.position += self.velocity * dt;
        // Now we update the lifespan of the particle;
        self.lifespan -= dt;
    }

    pub fn is_dead(&self) -> bool {
        self.lifespan <= 0.
    }

    pub fn set_position(&mut self, pos: Vector2D) {
        self.position = pos;
    }
}
