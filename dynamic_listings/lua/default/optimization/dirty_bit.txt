Player = {}

function Player:new(o)
    o = o or {
        speed = {0, 0},
        needs_update = false
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

-- ...
function Player:input()
    -- ...
    if self.right_key.is_Pressed then
        self.speed.x = self.speed.x + 1  -- Move right
        self.needs_update = true
        -- ...
    end
    if self.up_key.is_Pressed then
        self.speed.y = self.speed.y -100  -- Move up (jump)
        self.needs_update = true
    end
    -- ...
end

function Player:update(dt)
    if self.needs_update then
        -- Do Update instructions
        -- ...
    end
end
