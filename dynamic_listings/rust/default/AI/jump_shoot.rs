# //! ```cargo
# //! [dependencies]
# //! rand = "0.10"
# //! ```
#
# #![allow(dead_code, unused)]
struct Player {
    // ...
}

impl Player {
    fn register_shooting_observer(&self, callback: fn(&mut JumpingBoss)) {
        // Function used to register an observer that will be called when the
        // player shoots a projectile.
    }
}

struct JumpingBoss {
    player_shot: bool,
    on_ground: bool,
    y_velocity: f32,
}

impl JumpingBoss {
    pub fn new(x: f32, y: f32, player: &Player) {
        player.register_shooting_observer(Self::set_player_shot);
    }

    pub fn set_player_shot(&mut self) {
        self.player_shot = true;
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.y_velocity = -10.;
        }
    }

    pub fn update(&mut self, dt: f32) {
        // ...
        if self.player_shot {
            if rand::random_range(1..=5) == 1 {
                // Jump only 20% of the times the player shoots
                self.jump();
            }
        }
        // We reset player_shot to false, if we didn't the boss would jump
        // a lot more often than 20% of the time
        self.player_shot = false;
        // ...
    }
}
