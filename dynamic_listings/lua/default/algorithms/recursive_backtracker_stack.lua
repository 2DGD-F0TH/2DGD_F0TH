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
    o.carved_passages = {}
    return o
end

-- ...
-- Same as the previous example
-- ...

function Maze:draw_maze(x, y)
    --[[
    Draws a maze using the recursive backtracker algorithm and
    system stack recursion.

    This version will dig walls 2 cells at a time, thus leaving some cells
    acting as "walls" instead of having a more complex structure.
    ]]
    -- First thing, we push our cell into our "carved passages",
    -- this will simulate the system stack
    table.insert(self.carved_passages,{x,y})
    -- When the "carved_passages" array is empty, we are back at the start of the maze
    local stored_x = x
    local stored_y = y
    while (#self.carved_passages ~= 0) do
        -- We dig a path in the current cell
        self.make_path(stored_x, stored_y)
        -- We create a list of available directions (x and y)
        local directions = {
            {x=1, y=0},
            {x=-1, y=0},
            {x=0, y=1},
            {x=0, y=-1}
        }
        -- And we shuffle them (we need a custom-made helper function for that)
        directions = shuffle(directions)

        local next_x = nil
        local next_y = nil

        -- While there is at least one direction available
        while (#directions ~= 0) do
            -- We take the last item in our directions list (which is random)
            local direction_to_try = table.remove(directions, #directions)

            -- Calculate the new node's coordinates using the chosen direction.
            -- We are doubling the movement in each direction, so some cells
            -- can act as walls
            next_x = stored_x + (direction_to_try.x * 2)
            next_y = stored_y + (direction_to_try.y * 2)

            -- If the node we found is a wall, it means we didn't visit it
            -- (and it's inside our maze boundaries)
            if (self.is_wall(next_x, next_y)) then
                -- We have found a new node to dig towards

                -- Since we are moving 2 cells at a time, we need to carve
                -- the cell that "links" our "current node" and the "next node"
                local link_cell_x = stored_x + direction_to_try.x
                local link_cell_y = stored_y + direction_to_try.y
                self.make_path(link_cell_x, link_cell_y)

                -- Repeat the carving process with the new coordinates
                -- we do this by pushing into the stack
                table.insert(self.carved_passages, {x=next_x, y=next_y})
            end
        end
        -- If all available directions are exhausted, we pop
        -- one layer from our stack: we go back one step
        if (#directions == 0) then
            local popped_vector = table.remove(self.carved_passages, #self.carved_passages)
            stored_x = popped_vector.x
            stored_y = popped_vector.y
        else
            -- If not, we continue
            stored_x = next_x
            stored_y = next_y
        end
    end
end
