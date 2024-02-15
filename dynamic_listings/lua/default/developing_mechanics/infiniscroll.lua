-- ...
local background_x_offset = 0.0  -- The x offset of the background
BACKGROUND_X_SIZE = 512  -- The horizontal size of the background
LOOP_POINT = 256  -- The horizontal loop point of the image
DISTANCE_FACTOR = 0.5  -- The background moves at half the player speed


local function update(dt)
    -- ...
    -- In case we're moving right, the background scrolls left slightly
    if (player.speed_x > 0) then
        -- Update player's position and state
        -- ...
        -- Update the background position
        background_x_offset = background_x_offset - player.speed_x * DISTANCE_FACTOR * dt
        -- If we passed the image's loop point
        if (background_x_offset <= - LOOP_POINT) then
            -- We reset the coordinates, keeping note of the remainder
            background_x_offset = background_x_offset % LOOP_POINT
        end
    end
    -- In case we're moving left, the background scrolls right slightly
    if (player.speed_x < 0) then
        -- Update player's position and state
        -- ...
        -- Update the background position
        background_x_offset = background_x_offset - player.speed_x * DISTANCE_FACTOR * dt
        if (background_x_offset >= 0) then
            -- We reset the coordinates, keeping note of the remainder, just backwards
            background_x_offset = background_x_offset - BACKGROUND_X_SIZE
        end
    end
end

local function draw()
    -- ...
    -- Draw the background
    screen.draw(background, {x=background_x_offset, y=0})
    -- ...
end
