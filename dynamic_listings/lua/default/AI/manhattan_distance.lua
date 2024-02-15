Tile = {
    x = 0,
    y = 0
}

local function manhattan_distance(start, goal)
    return math.abs(start.x - goal.x) + math.abs(start.y - goal.y)
end
