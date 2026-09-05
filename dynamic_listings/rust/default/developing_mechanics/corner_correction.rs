# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
# struct Rectangle {
#     right: f32,
#     left: f32,
#     bottom: f32,
#     top: f32,
# }
# struct Block {
#     rect: Rectangle,
# }
# #[derive(Default)]
# struct Player {
#     velocity: Vector2D,
#     position: Vector2D,
#     width: f32,
# }
# impl Player {
#     fn find_collisions(&self, _: &[Block]) -> Vec<Block> { todo!() }
#     fn is_jumping(&self) -> bool { todo!() }
# }
# struct Game;
# impl Game {
fn update(&mut self, dt: f32) {
# let mut player = Player::default();
# let blocks = [];
    // Using the brute force checking for simplicity
    for block in player.find_collisions(&blocks) {
        // ...
        if player.is_jumping() {
            // We are jumping, we need to check if we are ascending
            // this way we will avoid "bonking our head" on a pixel
            if player.velocity.y < 0. {
                // We know we are ascending, let's check how far we are
                // from the borders and react accordingly
                if player.position.x > block.rect.right - 5. {
                    // The player's left side is penetrating the block by
                    // less than 5 pixels, let's correct it
                    player.position.x = block.rect.right;
                } else if player.position.x + player.width < block.rect.left + 5. {
                    // The player's right side is penetrating the block by
                    // less than 5 pixels, let's correct it
                    player.position.x = block.rect.left - player.width;
                } else {
                    // The player is totally colliding with (bonking its head on) the block,
                    // without need for corner correction, let's just act normally
                    player.velocity.y = 0.;
                    player.position.y = block.rect.bottom;
                }
            }
        }
    }
}
# }
