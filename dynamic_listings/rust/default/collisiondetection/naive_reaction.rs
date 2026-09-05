# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
#
# #[derive(Default)]
# struct Player {
#     position: Vector2D,
#     speed: Vector2D,
#     x_speed: f32,
#     y_speed: f32,
# }
# #[derive(Default)]
# struct Object {
#     rectangle: Rectangle,
# }
# #[derive(Default)]
# struct Rectangle {
#     left: f32,
#     right: f32,
#     top: f32,
#     bottom: f32,
# }
# fn collision(_: &Player, _: &Object) -> bool { todo!() }
#
# impl Player {
// Naive collision reaction with rectangles
fn update(&mut self, dt: f32) {
# let mut player = Player::default();
# let object = Object::default();
    // ...
    player.position = player.position + player.speed * dt;
    // Refer to your favourite collision detection and broad/fine passes
    if collision(&player, &object) {
        if player.x_speed > 0. {
            // going right
            player.position.x = object.rectangle.left; // reset position
            player.x_speed = 0.; // stop the player
        }
        if player.x_speed < 0. {
            // going left
            player.position.x = object.rectangle.right; // reset position
            player.x_speed = 0.; // stop the player
        }
    }
    // Again, refer to your favourite collision detection and broad/fine passes
    if collision(&player, &object) {
        if player.y_speed > 0. {
            // going down
            player.position.y = object.rectangle.top; // reset position
            player.y_speed = 0.; // stop the player
        }
        if player.y_speed > 0. {
            // going up
            player.position.y = object.rectangle.bottom; // reset position
            player.y_speed = 0.; // stop the player
        }
        // ...
    }
}
# }
