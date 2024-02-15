Point = {}

function Point:new(o)
    -- This is an example constructor
    o = o or {x=0, y=0}
    setmetatable(o, self)
    self.__index = self
    return o
end

Rectangle = {}

function Rectangle:new(o)
    -- This is an example constructor
    o = o or {corner=Point:new(), width=0, height=0}
    setmetatable(o, self)
    self.__index = self
    return o
end

local function rect_rect_collision(A, B)
    if ((A.corner.x < B.corner.x + B.width) and
       (A.corner.x + A.width > B.corner.x) and
       (A.corner.y < B.corner.y + B.height) and
       (A.corner.y + A.height > A.corner.y)) then
        return true
    else
        return false;
    end
end
