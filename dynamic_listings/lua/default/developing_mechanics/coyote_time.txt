Player = {}

function Player:new(o)
    o = o or {
        coyote_time_started = false,
        coyote_time = 0.0,
        onground = false,
        has_jumped = false
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Player:update(dt)
    -- ...
    if (self.onground) then
        -- Do stuff when player is on ground
        -- ...
    else
        if (not self.has_jumped) then
            -- Player is not on ground and has not jumped, the player's falling
            if (not self.coyote_time_started) then
                self.coyote_time_started = true
                self.coyote_time = 5
            else
                self.coyote_time = self.coyote_time - dt
            end
        end
    end
end

function Player:jump()
    -- This function takes care of jumping
    -- ...
    if (self.coyote_time > 0) then
        -- Do Jump
    end
    -- ...
end
