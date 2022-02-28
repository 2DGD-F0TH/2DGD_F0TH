-- ...
local function polygon_point(poly, point)
    -- First of all, we get the polygon's bounding box
    local bounding_box = poly:calculate_bounding_box()
    -- Then we do a simple point vs. rectangle check
    if (not point_rectangle(bounding_box, point)) then
        -- We are not even in the bounding box, we can't collide
        return false
    end
    -- If instead we are in the bounding box, we need to get the "fan triangulation"
    local triangles = poly.do_fanning()
    -- Now we check, for each triangle, if the point collides
    for i = 1, #triangles do
        if (point_triangle(triangles[i], point)) then
            -- We found the "slice" of the polygon that the point collides with
            return true
        end
    end
    -- If we pass all triangles without a hit, we are in the bounding box
    -- but outside the polygon, that's the worst case, and we are not colliding
    return false
end
