# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
# struct Texture;
struct Particle {
    /*
     * This is a simple particle class, now it has some mass
     * and a force application function
     */
    // ...
    acceleration: Vector2D,
    // ...
    mass: f32,
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
        m: Option<f32>,
    ) -> Self {
        // We prepare the particle for usage the same way as earlier
        Self {
            // ...
#           acceleration,
            mass: m.unwrap_or(1.),
        }
    }

    pub fn apply_force(&mut self, force: Vector2D) {
        let da = force / self.mass;
        self.acceleration += da;
    }
}
