Color = {}

function Color:new(o)
    -- This is an example constructor
    o = o or {colorData={}}
    setmetatable(o, self)
    self.__index = self
    return o
end

function Color:isWhite()
    -- ...
end

Bitmask = {}

function Bitmask:new(o)
    -- This is an example constructor
    o = o or {Color:new()}
    setmetatable(o, self)
    self.__index = self
    return o
end

function Bitmask:getColor()
    --- ...
end

Sprite = {}

function Bitmask:new(o)
    -- This is an example constructor
    o = o or {
        bitmask = Bitmask:new(),
        x = 0,
        y = 0,
        width = 16,
        height = 16
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

local function pixel_perfect_collision(A, B)
    -- Calculate the intersecting rectangle to limit checks
    local x1 = math.max(A.x, B.x)
    local x2 = math.min((A.x + A.width), (B.x + B.width))

    local y1 = math.max(A.y, B.y)
    local y2 = math.min((A.y + A.height), (B.y + B.height))

    -- For each pixes in the intersecting rectangle, let's check
    for y = y1, y2 do
        for x = x1, x2 do
            local a = A.bitmask.getColor(x - A.x, y - A.y)
            local b = B.bitmask.getColor(x - B.x, y - B.y)

            if (a.isWhite() and b.isWhite()) then
                return true
            end
        end
    end

    -- If no collision is occurred by the end of the checking, we're safe
    return false
end
