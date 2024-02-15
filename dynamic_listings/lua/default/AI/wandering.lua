DIRECTIONS = {'NORTH', 'EAST', 'SOUTH', 'WEST'}

AIEntity = {}

function AIEntity:new(o)
    -- This is an example constructor
    o = o or {
        -- 1=North, 2=East, ...
        forward_direction_index = 1,
        current_cell = {2,1}
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function AIEntity:get_adjacent_cell(cell, direction)
    -- Returns the adjacent cell in said direction
    local cell_copy = cell:copy()
    if (direction == 'NORTH') then
        cell_copy.y = cell_copy.y - 1
    end
    if (direction == 'SOUTH') then
        cell_copy.y = cell_copy.y + 1
    end
    if (direction == 'WEST') then
        cell_copy.x = cell_copy.x - 1
    end
    if (direction == 'EAST') then
        cell_copy.x = cell_copy.x + 1
    end
    return cell_copy
end

function AIEntity:is_valid(cell)
    -- Returns true if the cell is valid, aka
    -- does not have a wall and does not go backwards
    if (cell:is_wall()) then
        -- The cell is a wall
        return false
    end
    local backwards_cell = self.get_adjacent_cell(self.current_cell, DIRECTIONS[(self.forward_direction_index + 2) % 4])
    if (cell == backwards_cell) then
        -- We're going backwards, we don't want that
        return false
    end
    -- In all other cases, it's valid
    return true
end

function AIEntity:update(dt)
    -- ...
    -- Choose a random direction
    local rand_idx = math.random(1,4)
    local chosen_direction = DIRECTIONS[rand_idx]
    local i = 0
    local next_cell = self.get_adjacent_cell(self.current_cell, chosen_direction)
    while (not self.is_valid(next_cell) and i ~= 4) do
        chosen_direction = DIRECTIONS[(rand_idx + 1) % 4]
        next_cell = self.get_adjacent_cell(self.current_cell, chosen_direction)
        i = i + 1
    end
    if (i == 4) then
        -- We exhausted the possibilities, go backwards
        chosen_direction = DIRECTIONS[(self.forward_direction_index + 2) % 4]
        next_cell = self.get_adjacent_cell(self.current_cell, chosen_direction)
    end
    -- Move
    self.move_to(next_cell)
end
