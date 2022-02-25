-- Let's define a circle class/structure
Circle = {}

function Circle:new(o)
    -- This is an example constructor
    o = o or {center=Point:new(), radius=0}
    setmetatable(o, self)
    self.__index = self
    return o
end

local function distance(A, B)
    -- Calculates the distance between two points
    return math.sqrt((A.x - B.x)^2 + (A.y - B.y)^2)
end

local function circle_circle_collision(A, B)
    return distance(A.center, B.center) <= A.radius + B.radius
end
