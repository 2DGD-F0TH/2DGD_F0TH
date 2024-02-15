Tile = {
    x = 0,
    y = 0
}

local function euclidean_distance(start, goal)
    return math.sqrt((start.x - goal.x)^2 + (start.y - goal.y)^2)
end
