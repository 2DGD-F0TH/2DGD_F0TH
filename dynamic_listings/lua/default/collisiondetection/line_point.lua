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

local function distance(A, B)
    -- Calculates the distance between two points
    return math.sqrt((A.x - B.x)^2 + (A.y - B.y)^2)
end

local function line_point_collision(pt, ln)
    -- First, let's calculate the length of the line
    local length = distance(ln.A, ln.B)
    -- Now let's calculate the distance between the point pt
    -- and the point "A" of the line
    local pt_a = distance(ln.A, pt)
    -- Same Goes for the distance between pt and "B"
    local pt_b = distance(ln.B, pt)
    -- Now for the detection
    if (pt_a + pt_b == length) then
        return true
    else
        return false
    end
end
