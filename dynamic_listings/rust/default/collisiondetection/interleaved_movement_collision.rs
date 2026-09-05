# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
# fn collision(player: &Player, object: &Object) -> bool {
#     todo!()
# }
# struct Player {
#     position: Vector2D,
#     x_speed: f32,
#     y_speed: f32,
# }
# #[derive(Default)]
# struct Rectangle {
#     left: f32,
#     right: f32,
#     bottom: f32,
#     top: f32,
# }
# #[derive(Default)]
# struct Object {
#     rectangle: Rectangle,
# }
# struct Game {
#     player: Player,
# }
# impl Game {
// Interleaving movement and collision reaction with rectangles
fn update(&mut self, dt: f32) {
# let object = Object::default();
    // ...
    self.player.position.x = self.player.position.x + self.player.x_speed * dt;
    // Refer to your favourite collision detection and broad/fine passes
    if collision(&self.player, &object) {
        if self.player.x_speed > 0. {
            // going right
            self.player.position.x = object.rectangle.left; // reset position
            self.player.x_speed = 0.; // stop the player
        }
        if self.player.x_speed < 0. {
            // going left
            self.player.position.x = object.rectangle.right; // reset position
            self.player.x_speed = 0.; // stop the player
        }
    }
    self.player.position.y = self.player.position.y + self.player.y_speed * dt;
    // Again, refer to your favourite collision detection and broad/fine passes
    if collision(&self.player, &object) {
        if self.player.y_speed > 0. {
            // going down
            self.player.position.y = object.rectangle.top; // reset position
            self.player.y_speed = 0.; // stop the player
        }
        if self.player.y_speed > 0. {
            // going up
            self.player.position.y = object.rectangle.bottom; // reset position
            self.player.y_speed = 0.; // stop the player
        }
        // ...
    }
}
# }
