# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<i32>;
# struct Rectangle {
#     bottom: f32,
#     top: f32,
#     right: f32,
#     left: f32,
# }
# struct Entity {
#     speed: Vector2D,
#     rect: Rectangle,
# }
# fn overlap(_: &Entity, _: &Entity) -> Vector2D { todo!() }
fn solve_collision(player: &mut Entity, object: &Entity) {
    /*
     * This algorithm solves a collision between the player
     * and an unmovable object
     * We are assuming the player is moving
     */
    // The overlap will help us decide how to react
    let overlap: Vector2D = overlap(player, object);
    if overlap.x > overlap.y {
        // Y is the "shallow axis"
        if player.speed.y > 0 {
            // Player is going towards the bottom of screen
            player.rect.bottom = object.rect.top;
        } else {
            // Player is going towards the top of the screen
            player.rect.top = object.rect.bottom;
        }
    } else {
        // X is the "shallow axis"
        if player.speed.x > 0 {
            // Player is going right
            player.rect.right = object.rect.left;
        } else {
            // Player is going left
            player.rect.left = object.rect.left;
        }
    }
}
