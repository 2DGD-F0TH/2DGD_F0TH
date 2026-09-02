# #![allow(dead_code, unused)]
# struct Screen {
# }
# impl Screen {
#     fn draw(&self, _: &str, _: (f32, f32)) {}
# }
# struct Player {
#     speed_x: f32,
# }
# struct Game {
#     screen: Screen,
#     player: Player,
#     background_x_offset: f32,
# }
# impl Game {
fn update(&mut self, dt: f32) {
    let background_x_offset: f32 = 0.;  // The x offset of the background
    const BACKGROUND_X_SIZE: f32 = 512.;  // The horizontal size of the background
    const LOOP_POINT: f32 = 256.0;  // The horizontal loop point of the image
    const DISTANCE_FACTOR: f32 = 0.5;  // The background moves at half the player speed

    //...
    // In case we're moving right, the background scrolls left slightly
    if self.player.speed_x > 0. {
        // Update player's position and state
        //...
        // Update the background position
        self.background_x_offset = self.background_x_offset - self.player.speed_x * DISTANCE_FACTOR * dt;
        // If we passed the image's loop point
        if self.background_x_offset <= -LOOP_POINT {
            // We reset the coordinates, keeping note of the remainder
            self.background_x_offset = self.background_x_offset % LOOP_POINT;
        }
    }
    // In case we're moving left, the background scrolls right slightly
    if self.player.speed_x < 0. {
        // Update player's position and state
        //...
        // Update the background position
        self.background_x_offset = self.background_x_offset - self.player.speed_x * DISTANCE_FACTOR * dt;
        if self.background_x_offset >= 0. {
            // We reset the coordinates, keeping note of the remainder, just backwards
            self.background_x_offset = self.background_x_offset - BACKGROUND_X_SIZE;
        }
    }
}

fn draw(&self) {
#   let background = "";
    //...
    // Draw the background
    self.screen.draw(background, (self.background_x_offset, 0.));
    //...
}
# }
