Point = {}

function Point:new(o)
    -- This is an example constructor
    o = o or {x=0, y=0}
    setmetatable(o, self)
    self.__index = self
    return o
end

Line = {}

function Line:new(o)
    -- This is an example constructor
    o = o or {A=Point:new(), B=Point:new()}
    setmetatable(o, self)
    self.__index = self
    return o
end

Circle = {}

function Circle:new(o)
    -- This is an example constructor
    o = o or {center=Point:new(), radius=0}
    setmetatable(o, self)
    self.__index = self
    return o
end

-- ...

local function line_circle_collision(circle, line)
    local collides_A = circle_point_collision(circle, line.A)
    local collides_B = circle_point_collision(circle, line.B)
    if (collides_A or collides_B) then
        return true
    end
    -- ...
end
