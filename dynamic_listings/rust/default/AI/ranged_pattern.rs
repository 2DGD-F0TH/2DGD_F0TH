struct Boss {
    player: Player,
    player_too_close: bool,
    base_movement_velocity: f32,
    too_close_space: f32,
    velocity: Vector2D,
    position: Vector2D,
}

impl Boss {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            player_too_close: false,
            base_movement_velocity: 10.,
            too_close_space: 20.,
            velocity: Vector2D::default(),
            position: Vector2D::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // ...
        if (self.player.position.x - self.position.x).abs() < self.too_close_space {
            if (self.player.position.y - self.position.y).abs() < self.too_close_space {
                // The player is too close
                if rand::random_range(1..=5) == 1 {
                    // Add a bit of randomization
                    self.player_too_close = true;
                }
            }
        }
        // We're using a variable to preserve the "too close" state between frames
        if self.player_too_close {
            // The player is too close, make some distance
            let distance = self.position - self.player.position;
            // Make it a direction
            let direction = distance.normalise();
            // is the direction the boss should go now, transfer it to velocity
            self.velocity = direction * self.base_movement_velocity;
        }
        // ...
        // The boss and player now have moved, let's see if they're far enough
        // ...
        if (self.player.position.x - self.position.x).abs() > self.too_close_space {
            if (self.player.position.y - self.position.y).abs() > self.too_close_space {
                // The player is far enough now
                self.player_too_close = false;
            }
        }
    }
}
