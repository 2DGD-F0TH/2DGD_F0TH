-- ...
JUMPBUFFERTIME = 5.0
-- ...
local function update(dt)
    -- ...
    if (CONTROLS.jump.isPressed()) then
        player.hasBufferedJump = true
        player.jumpBufferCountdown = JUMPBUFFERTIME
    end
    -- Take note on how this piece is outside the "jump is pressed" section
    if (player.hasBufferedJump) then
        player.jumpBufferCountdown = player.jumpBufferCountdown - dt
    end
    if (player.on_ground) then
        if (player.jumpBufferCountdown > 0.0) then
            -- Jump
            player.jumpBufferCountdown = 0.0
            player.hasBufferedJump = false
        end
    end
    -- ...
end
