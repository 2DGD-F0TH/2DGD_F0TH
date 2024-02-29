TiledPlayer = {}

function TiledPlayer:new(o)
    o = o or {
        offset = Vector2D:new({0,0}),
        current_position = Vector2D:new({10,10}),
        next_position = Vector2D:new({10,10})
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function TiledPlayer:update(dt)
    -- ...
    -- Check which direction is the player going
    if (KEYBOARD.Up_Arrow_Pressed) then
        self.offset.y = -1
    end
    if (KEYBOARD.Down_Arrow_Pressed) then
        self.offset.y = 1
    end
    if (KEYBOARD.Right_Arrow_Pressed) then
        self.offset.x = 1
    end
    if (KEYBOARD.Left_Arrow_Pressed) then
        self.offset.x = -1
    end
    -- Get the destination tile
    self.next_position = self.current_position + self.offset
    -- Is the tile a wall?
    if (not MAP.get_tile(self.next_position).isWall()) then
        -- No, move the player to the new tile
        self.current_position = self.next_position
    end
    -- ...
end
