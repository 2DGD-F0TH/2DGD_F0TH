# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
# #[derive(Clone)]
# struct Player {
#     y: f32,
#     x: f32,
#     on_ground: bool,
#     width: f32,
#     height: f32,
# }
# impl std::ops::AddAssign<Vector2D> for Player {
#     fn add_assign(&mut self, rhs: Vector2D) {
#         todo!()
#     }
# }
# fn snapshot_recation(player_instance: &mut Player, velocity: Vector2D, dt: f32) {
# let colliding_blocks: Vec<Player> = Vec::new();
// Snapshot collision reaction
// All the sprite origins are on the top-left corner of the entity
let snapshot = player_instance.clone();  // The "snapshot"

// Update the player_instance here
*player_instance += velocity * dt;

// Now check for collisions
// ...
// Considering "colliding_blocks" as the list of blocks colliding with player
for block in colliding_blocks {
    if (snapshot.y >= block.y + block.height) && (player_instance.y < block.y + block.height) {
        // We are coming on the block from below, react accordingly
        // Ignoring this reaction will allow players to phase through blocks when coming from below
        player_instance.y = block.y + block.height;
    }

    if (snapshot.y + snapshot.height <= block.y) && (player_instance.y + snapshot.height > block.y) {
        // We are coming on the block from above
        player_instance.y = block.y;
        player_instance.on_ground = true;
    }

    if (snapshot.y + snapshot.width <= block.x) && (player_instance.x > block.x) {
        // We are coming on the block from left
        player_instance.x = block.x - player_instance.width;
    }

    if (snapshot.y >= block.x + block.width) && (player_instance.x < block.x + block.width) {
        // We are coming on the block from right
        player_instance.x = block.x + block.width;
    }
}
# }
