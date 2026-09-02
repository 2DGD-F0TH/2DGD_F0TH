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
    // The particle texture
    texture: Texture,
    // Position, velocity and acceleration on the 2D plane
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,
    // Lifespan of the particle
    lifespan: f32,
    // Current angle of rotation, and relative velocity and acceleration
    rotation: f32,
    angular_velocity: f32,
    angular_acceleration: f32,

    // Initial Status, for resetting
    initial_velocity: Vector2D,
    initial_rotation: f32,
    initial_a_vel: f32,
    initial_lifespan: f32,
}

impl Particle {
    pub fn new(
        texture: Texture,
        position: Vector2D,
        velocity: Vector2D,
        acceleration: Vector2D,
        lifespan: Option<f32>,
        rot: Option<f32>,
        a_vel: Option<f32>,
        a_accel: Option<f32>,
    ) -> Self {
        Self {
            // We prepare the particle for usage
            texture,
            position,
            velocity,
            acceleration,
            lifespan: lifespan.unwrap_or(2_000.), // About 2 seconds by default
            rotation: rot.unwrap_or_default(),
            angular_velocity: 0.,
            angular_acceleration: a_accel.unwrap_or_default(),
            // We also prepare the reset variables: the position will be set by the emitter
            initial_lifespan: lifespan.unwrap_or(2_000.),
            initial_velocity: velocity,
            initial_a_vel: a_vel.unwrap_or_default(),
            initial_rotation: rot.unwrap_or_default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // We update the velocity (assuming dt is in milliseconds)
        self.velocity += self.acceleration;
        // Then the position
        self.position += self.velocity * dt;
        // Then the rotation
        self.angular_velocity += self.angular_acceleration;
        // Wrap to zero when at 360 degrees
        self.rotation = (self.rotation + self.angular_velocity * dt) % 360.;
        // Now we update the lifespan of the particle;
        self.lifespan -= dt;
    }

    pub fn is_dead(&self) -> bool {
        // Returns a boolean representing if the particle is dead
        self.lifespan <= 0.
    }

    pub fn reset(&mut self) {
        // This function resets the initial status of the particle
        self.velocity = self.initial_velocity;
        self.rotation = self.initial_rotation;
        self.angular_velocity = self.initial_a_vel;
        self.lifespan = self.initial_lifespan;
    }
}
