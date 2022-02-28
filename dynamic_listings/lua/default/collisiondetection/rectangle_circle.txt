Point = {
    -- Rewritten as a memo
}

Rectangle = {}

function Rectangle:new(o)
    -- This is an example constructor
    o = o or {corner=Point:new(), width=0, height=0}
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

local function circle_rectangle_collision(circ, rect)
    -- Detects a collision between a circle and a rectangle

    -- These variables are used as the coordinates we should test against
    -- They are temporarily set to the circle center's coordinates for a reason we'll see soon
    local tx = circ.center.x
    local ty = circ.center.y

    -- Let's detect which edge to test against on the x axis
    if (circ.center.x < rect.corner.x) then
        -- We're at the left of the rectangle, test against the left side
        tx = rect.corner.x
    elseif (circ.center.x > rect.corner.x + rect.width) then
        -- We're at the right of the rectangle, test against the right side
        tx = rect.corner.y + rect.width
    end

    -- Same thing on the vertical axis
    if (circ.center.y < rect.corner.y) then
        -- We're above the rectangle, test against the top side
        ty = rect.corner.y
    elseif (circ.center.y > rect.corner.y + rect.height) then
        -- We're below the rectangle, test against the bottom side
        ty = rect.corner.y + rect.height
    end

    -- Let's get the distance between the testing coordinates and the circle center
    local distanceX = circ.center.x - tx
    local distanceY = circ.center.y - ty
    local distance = math.sqrt(distanceX^2 + distanceY^2)

    -- Note that if the center of the circle is inside the rectangle, the testing coordinates will be the circle's center itself, thus the next conditional will always return true

    if (distance <= circ.radius) then
        return true
    end

    -- Default to false in case no collision occurs
    return false
end
