// Snapshot collision reaction
// All the sprite origins are on the top-left corner of the entity
let snapshot = player_instance.copy();  // The "snapshot"

// Update the player_instance here
player_instance.position = player_instance.position + (velocity * dt);

// Now check for collisions
// ...
for (const block of player_instance.get_collision_list()){
    if ((snapshot.y >= block.y + block.height) && (player_instance.y < block.y + block.height)){
        // We are coming on the block from below, react accordingly
        // Ignoring this reaction will allow players to phase through blocks when coming from below
        player_instance.position.y = block.y + block.height;
    }

    if ((snapshot.y + snapshot.height <= block.y) && (player_instance.y + snapshot.height > block.y)){
        // We are coming on the block from above
        player_instance.position.y = block.y;
        player_instance.on_ground = true;
    }

    if ((snapshot.y + snapshot.width <= block.x) && (player_instance.x > block.x)){
        // We are coming on the block from left
        player_instance.position.x = block.x - player_instance.width;
    }

    if ((snapshot.y >= block.x + block.width) && (player_instance.x < block.x + block.width)){
        // We are coming on the block from right
        player_instance.position.x = block.x + block.width;
    }
}
