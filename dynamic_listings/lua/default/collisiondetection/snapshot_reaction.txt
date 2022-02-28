-- Snapshot collision reaction
-- All the sprite origins are on the top-left corner of the entity
local snapshot = player_instance:copy()  -- The "snapshot"

-- Update the player_instance here
player_instance.position = player_instance.position + (velocity * dt)

-- Now check for collisions
-- ...
local colliding_blocks = player:getCollisions()
for i = 1, #colliding_blocks do
    local block = colliding_blocks[i]
    if ((snapshot.y >= block.y + block.height) and (player_instance.y < block.y + block.height)) then
        -- We are coming on the block from below, react accordingly
        -- Ignoring this reaction will allow players to phase through blocks when coming from below
        player_instance.position.y = block.y + block.height
    end

    if ((snapshot.y + snapshot.height <= block.y) and (player_instance.y + snapshot.height > block.y)) then
        -- We are coming on the block from above
        player_instance.position.y = block.y
        player_instance.on_ground = true
    end

    if ((snapshot.y + snapshot.width <= block.x) and (player_instance.x > block.x)) then
        -- We are coming on the block from left
        player_instance.position.x = block.x - player_instance.width
    end

    if ((snapshot.y >= block.x + block.width) and (player_instance.x < block.x + block.width)) then
        -- We are coming on the block from right
        player_instance.position.x = block.x + block.width
    end
end
