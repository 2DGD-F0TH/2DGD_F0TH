local function lineLineCollision(x1, y1, x2, y2, x3, y3, x4, y4)
    -- our previous implementation of the line/line collision detection
    -- ...
end

local function pointRectCollision(x1, y1, rectx, recty, rectwidth, rectheight)
    -- our previous implementation of a point/rectangle collision detection
    -- ...
end

local function lineRectangleCollision(x1, y1, x2, y2, rectx, recty, rectwidth, rectheight)
    -- If we want to test if a line is completely inside of a rect, we just need
    -- to see if any of its endpoints is inside the rectangle
    if (pointRectCollision(x1, y1, rectx, recty, rectwidth, rectheight) or pointRectCollision(x2, y2, rectx, recty, rectwidth, rectheight)) then
        -- At least one of the ends of the segment is inside the rectangle
        return true
    end
    -- Now to test the rectangle against the line, if it's not completely inside
    local left = lineLineCollision(x1, y1, x2, y2, rectx, recty, rectx, recty + rectheight)
    local right = lineLineCollision(x1, y1, x2, y2, rectx + rectwidth, recty, rectx + rectwidth, recty + rectheight)
    local top = lineLineCollision(x1, y1, x2, y2, rectx, recty, rectx + rectwidth, recty)
    local bottom = lineLineCollision(x1, y1, x2, y2, rectx, recty + rectheight, rectx + rectwidth, recty + rectheight)

    if (left or right or top or bottom) then
        -- We hit one of the sides, we are colliding
        return true
    end

    -- In any other case, return false
    return false
end
