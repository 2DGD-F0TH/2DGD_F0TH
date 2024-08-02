# Snapshot collision reaction
# All the sprite origins are on the top-left corner of the entity
snapshot: Player = player_instance.copy()  # The "snapshot"

# Update the player_instance here
player_instance.x += (velocity.x * dt)
player_instance.y += (velocity.y * dt)

# Now check for collisions
...
for block in player_instance.collision_list():
    if (snapshot.y >= block.y + block.height) and (player_instance.y < block.y + block.height):
        # We are coming on the block from below, react accordingly
        # Ignoring this reaction will allow players to phase through blocks when coming from below
        player_instance.y = block.y + block.height

    if (snapshot.y + snapshot.height <= block.y) and (player_instance.y + snapshot.height > block.y):
        # We are coming on the block from above
        player_instance.y = block.y
        player_instance.on_ground = True

    if (snapshot.y + snapshot.width <= block.x) and (player_instance.x > block.x):
        # We are coming on the block from left
        player_instance.x = block.x - player_instance.width

    if (snapshot.y >= block.x + block.width) and (player_instance.x < block.x + block.width):
        # We are coming on the block from right
        player_instance.x = block.x + block.width
