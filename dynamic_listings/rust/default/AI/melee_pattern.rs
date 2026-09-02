# //! ```cargo
# //! [dependencies]
# //! rand = "0.10"
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
# struct Player {
#     position: Vector2D,
# }
struct Boss {
    player: Player,
    player_too_far: bool,
    base_movement_velocity: f32,
    too_far_space: f32,
    velocity: Vector2D,
    position: Vector2D,
}

impl Boss {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            player_too_far: false,
            base_movement_velocity: 10.,
            too_far_space: 30.,
            velocity: Vector2D::default(),
            position: Vector2D::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // ...
        if (self.player.position.x - self.position.x).abs() > self.too_far_space {
            if (self.player.position.y - self.position.y).abs() > self.too_far_space {
                // The player is too close
                if rand::random_range(1..=5) == 1 {
                    // Add a bit of randomization
                    self.player_too_far = true;
                }
            }
        }
        // We're using a variable to preserve the "too far" state between frames
        if self.player_too_far {
            // The player is too far, close in
            let distance = self.player.position - self.position;
            // Make it a direction
            let direction = distance.normalise();
            // is the direction the boss should go now, transfer it to velocity
            self.velocity = direction * self.base_movement_velocity;
        }
        // ...
        // The boss and player now have moved, let's see if they're close enough
        // ...
        if (self.player.position.x - self.position.x).abs() < self.too_far_space {
            if (self.player.position.y - self.position.y).abs() < self.too_far_space {
                // The player is close enough now
                self.player_too_far = false;
            }
        }
    }
}
