function pointRectCollision(x1, y1, rectx, recty, rectwidth, rectheight)
    -- We check if the point is inside the rectangle
    return x1 >= rectx and x1 <= rectx + rectwidth and y1 >= recty and y1 <= recty + rectheight
end
