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

local function distance(A, B)
    -- Calculates the distance between two points
    return math.sqrt((A.x - B.x)^2 + (A.y - B.y)^2)
end

local function line_point_collision(line, point)
    -- ...
end

local function circle_point_collision(circ, point)
    -- ...
end

local function line_circle_collision(circle, line)
    -- We check the ends first
    local collides_A = circle_point_collision(circle, line.A)
    local collides_B = circle_point_collision(circle, line.B)
    if (collides_A or collides_B) then
        return true
    end
    -- We pre-calculate "u", we'll use some variables for readability
    local x1 = line.A.x
    local x2 = line.B.x
    local xk = circle.center.x
    local y1 = line.A.y
    local y2 = line.B.y
    local yk = circle.center.y
    local u = ((xk - x1) * (x2 - x1) + (yk - y1) * (y2 - y1))/(distance(line.A, line.B))^2
    -- Now let's calculate the x and y coordinates
    local x = x1 + u * (x2 - x1)
    local y = y1 + u * (y2 - y1)
    -- "Reuse": we'll use some older functions, let's create a point, with the coordinates we found
    local P = Point:new({x=x, y=y})
    -- Let's check if the "closest point" we found is on the line
    if ((line_point_collision(line, P)) == false) then
        -- If the point is outside the line, we return false, because the ends have already been checked against collisions
        return false
    else
        -- Let's Reuse the Point/Circle Algorithm
        return circle_point_collision(circle, P)
    end
end
