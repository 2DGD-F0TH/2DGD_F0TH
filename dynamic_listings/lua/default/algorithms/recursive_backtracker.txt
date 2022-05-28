Maze = {}

function Maze:new(o)
    --
    -- A simple constructor
    --
    o = o or {
        width=0,
        height=0
    }
    setmetatable(o, self)
    self.__index = self
    -- Create a 2D matrix containing the maze data
    -- False = Path, True = Wall
    o.cells = {}
    for i = 1, o.width do
        o.cells[i] = {}
        for j = 1, o.height do
            o.cells[i][j] = true
        end
    end
    return o
end

function Maze:make_path(x, y)
    --[[
    Given a cell coordinates, sets the cell as a path
    ]]
    self.cells[y][x] = false
end

function Maze:make_wall(x, y)
    --[[
    Given a cell coordinates, sets the cell as a wall
    ]]
    self.cells[y][x] = true
end

function Maze:is_wall(x, y)
    --[[
    Returns true if the given cell coordinates are inside the maze
    boundaries and if the selected cell is a wall
    ]]
    -- Let's check if the coordinates are within the maze grid
    if (x >= 0) and (x < self.width) and (y >= 0) and (y < self.height) then
        -- if they are, then we can check if the cell is a wall
        return self.cells[y][x]
    else
        -- If we're outside the maze boundaries, we return false
        return false
    end
end

function Maze:draw_maze(x, y)
    --[[
    Draws a maze using the recursive backtracker algorithm and
    system stack recursion.

    This version will dig walls 2 cells at a time, thus leaving some cells
    acting as "walls" instead of having a more complex structure.
    ]]

    -- We dig a path in the current cell
    self.make_path(x, y)
    -- We create a list of available directions (x and y)
    local directions = {
        {x=1, y=0},
        {x=-1, y=0},
        {x=0, y=1},
        {x=0, y=-1}
    }
    -- And we shuffle them (we need a custom-made helper function for that)
    directions = shuffle(directions)

    -- While there is at least one direction available
    while (#directions ~= 0) do
        -- We take the last item in our directions list (which is random)
        local direction_to_try = table.remove(directions, #directions)

        -- Calculate the new node's coordinates using the chosen direction.
        -- We are doubling the movement in each direction, so some cells
        -- can act as walls
        local next_x = x + (direction_to_try.x * 2)
        local next_y = y + (direction_to_try.y * 2)

        -- If the node we found is a wall, it means we didn't visit it
        -- (and it's inside our maze boundaries)
        if (self.is_wall(next_x, next_y)) then
            -- We have found a new node to dig towards

            -- Since we are moving 2 cells at a time, we need to carve
            -- the cell that "links" our "current node" and the "next node"
            local link_cell_x = x + direction_to_try.x
            local link_cell_y = y + direction_to_try.y
            self.make_path(link_cell_x, link_cell_y)

            -- Repeat the carving process with the new coordinates
            self.draw_maze(next_x, next_y)
        end
    end
    -- If all available directions are exhausted, we return (thus popping
    -- one layer from the system stack: we go back one step)
    -- If we are back at the starting cell, the algorithm terminates
end
