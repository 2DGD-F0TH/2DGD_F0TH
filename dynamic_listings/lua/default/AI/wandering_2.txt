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

function AIEntity:get_available_directions(cell)
    -- Returns a list of available directions
    local result = {}
    for i = 1, #DIRECTIONS do
        if (self.is_valid(self.get_adjacent_cell(cell, DIRECTIONS[i]))) then
            table.insert(result, DIRECTIONS[i])
        end
    end
    return result
end

function AIEntity:update(dt)
    -- ...
    -- Get a list of the available directions
    local available_directions = self.get_available_directions(self.current_cell)
    local chosen_direction = "NORTH"  -- Just a default
    if (#available_directions == 0) then
        -- No directions are available, let's go back
        chosen_direction = DIRECTIONS[(self.forward_direction_index + 2) % 4]
    else
        -- Choose a random direction among the available ones
        local random_idx = math.random(1, #available_directions)
        chosen_direction = available_directions[random_idx]
    end
    -- Move
    local next_cell = self.get_adjacent_cell(self.current_cell, chosen_direction)
    self.move_to(next_cell)
end
