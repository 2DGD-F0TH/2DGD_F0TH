-- First, we prepare some useful functions
local function dot_product(u, v)
    return (u.x * v.x) + (u.y * v.y);
end

local function scale_vector(factor, v)
    return Vector2D({
        x = factor * v.x,
        y = factor * v.y
    });
end

local function magnitude(v)
    return math.sqrt(dot_product(v, v));
end

-- ...
if collides(obj1, obj2) then
    -- Here we know that obj1 and obj2 are colliding, and we assume
    -- they are moving

    -- Since the "position" field is a vector, we can easily calculate "ucoll"
    local ucoll = obj2.position - obj1.position;
    -- Now we calculate its relative unit vector
    local unit_ucoll = ucoll / magnitude(ucoll);
    -- Let's calculate the relative velocity of the objects, since
    -- the "velocity" field is a vector, that's easy
    local vrel = obj2.velocity - obj1.velocity;
    -- Now we calculate s
    local s = dot_product(unit_ucoll, vrel);
    -- If s > 0, we need to change the velocity of the objects
    if (s > 0) then
        local factor = dot_product(s, unit_ucoll);
        obj2.velocity = scale_vector(factor, obj2.velocity);
        obj1.velocity = scale_vector(factor, obj1.velocity);
    end
    -- ...
end
-- ...
