-- ...
local function line_polygon(line, poly)
    -- First of all, let's check if either of the line ends are inside the polygon
    -- This covers cases AB and CD
    if (polygon_point(poly, line.A)) then
        -- One of the ends is inside the polygon, we have a hit
        return true
    end
    if (polygon_point(poly, line.B)) then
        -- One of the ends is inside the polygon, we have a hit
        return true
    end
    -- Now we check for case EF
    for i = 0, #poly.vertices do
        -- We iterate through all the vertices
        local j = i + 1
        -- If we get to the end, we wrap around j
        if (j == #poly.vertices) then
            j = 0
        end
        local temp_line = Line:fromPoints(poly.vertices[i], poly.vertices[j])
        if (line_line_collsion(temp_line, line)) then
            return true
        end
    end
    if (line_line_collision(temp_line, line)) then
        return true
    end
    -- If none of the previous checks was triggered, we don't have a collision
    return false
end
