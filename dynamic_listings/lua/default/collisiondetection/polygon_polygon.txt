-- ...
local function polygon_polygon(p1, p2)
    -- First we do a polygon vs line check for all the edges
    for i = 0, #p2.vertices do
        local j = i + 1
        if (j == #p2.vertices) then
            -- Wrap around in case we get to the end
            j = 0
        end
        local temp_line = Line:fromPoints(p2.vertices[i], p2.vertices[j])
        if (polygon_line(p1, temp_line)) then
            -- We have a hit
            return true
        end
    end
    -- Now we check in case one polygon contains the other, we can just check a single vertex
    if (polygon_point(p1, p2.vertices[0]) or polygon_point(p2, p1.vertices[0])) then
        return true
    end
    -- None of the checks was triggered, there is no collision
    return false
end
