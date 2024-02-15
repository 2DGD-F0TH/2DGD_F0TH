Boss = {}

function Boss:new(o)
    o = o or {
        player = Player:new(),
        player_too_far = false,
        base_movement_velocity = 10,
        too_far_space = 30,
        velocity = Vector2D:new(),
        position = Vector2D:new(),
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Boss:update(dt)
    -- ...
    if (math.abs(self.player.position.x - self.position.x) > self.too_far_space) then
        if (math.abs(self.player.position.y - self.position.y) > self.too_far_space) then
            -- The player is too close
            if (math.random(1, 5) == 1) then
                -- Add a bit of randomization
                self.player_too_far = true
            end
        end
    end
    -- We're using a variable to preserve the "too far" state between frames
    if (self.player_too_far) then
        -- The player is too far, close in
        local distance = self.player.position - self.position
        -- Make it a direction
        local direction = distance.normalize()
        -- self is the direction the boss should go now, transfer it to velocity
        self.velocity = direction * self.base_movement_velocity
    end
    -- ...
    -- The boss and player now have moved, let's see if they're close enough
    -- ...
    if (math.abs(self.player.position.x - self.position.x) < self.too_far_space) then
        if (math.abs(self.player.position.y - self.position.y) < self.too_far_space) then
            -- The player is close enough now
            self.player_too_far = false
        end
    end
end
