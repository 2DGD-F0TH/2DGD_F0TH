-- Interleaving movement and collision reaction with rectangles
local function update(dt)
    -- ...
    player.position.x = player.position.x + player.x_speed * dt
    -- Refer to your favourite collision detection and broad/fine passes
    if (collision(player, object)) then
        if (player.x_speed > 0) then  -- going right
            player.position.x = object.rectangle.left  -- reset position
            player.x_speed = 0  -- stop the player
        end
        if (player.x_speed < 0) then  -- going left
            player.position.x = object.rectangle.right -- reset position
            player.x_speed = 0  -- stop the player
        end
    end
    player.position.y = player.position.y + player.y_speed * dt
    -- Again, refer to your favourite collision detection and broad/fine passes
    if (collision(player, object)) then
        if (player.y_speed > 0) then  -- going down
            player.position.y = object.rectangle.top  -- reset position
            player.y_speed = 0  -- stop the player
        end
        if (player.y_speed > 0) then  -- going up
            player.position.y = object.rectangle.bottom  -- reset position
            player.y_speed = 0  -- stop the player
        end
    end
    -- ...
end
