local function is_collision(A, B)
    -- Defines how two items collide (being circles, this could be a difference of radii)
    -- ...
end

local items = [item1, item2, ...]

local function get_colliding_items(items_to_check)
    local colliding_items = {}

    for i = 1, #items_to_check do
        A = items_to_check[i]
        for j = 1, #items_to_check do
            B = items_to_check[j]
            if (A ~= B) then
                -- We avoid checking if an item collides with itself, for obvious reasons
                if is_collision(A, B) then
                    -- We keep together the pair of items that collided
                    colliding_items.add({A, B})
                end
            end
        end
    end
    return colliding_items
end
