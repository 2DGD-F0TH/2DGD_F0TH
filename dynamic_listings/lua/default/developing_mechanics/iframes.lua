INVINCIBILITY_TIME = 0.75  -- Seconds of invincibility
-- ...
function Player:update(dt)
    self.inv_time = 0
    -- ...
    if (self.inv_time <= 0) then
        -- Check for collision
        -- ...
        -- Collision has been detected here, we have a hit
        self.inv_time = INVINCIBILITY_TIME  -- Start of the invincibility period
    else
        -- We are currently invincible
        self.inv_time = self.inv_time - dt  -- We decrease the invincibility time
    end
end
