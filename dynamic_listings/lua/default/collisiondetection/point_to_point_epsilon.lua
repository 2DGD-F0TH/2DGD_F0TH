local function point_collision(A, B)
    local epsilon = 0.0001; -- Let's take a sufficiently low value
    -- If both coordinates are "close enough", we trigger a collision.
    -- We take the absolute value, just in case some subtractions end up being negative.
    return math.abs(A.x - B.x) <= epsilon and math.abs(A.y - B.y) <= epsilon;
end
