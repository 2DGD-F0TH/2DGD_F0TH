Grid_2D = {}

function Grid_2D:new(o)
    -- This is an example constructor
    o = o or {
        grid = {},
        width = 0,
        height = 0
    }
    setmetatable(o, self)
    self.__index = self
    return o
end


function Grid_2D:getCell(row, col)
    -- Gets a cell from the 2D Grid
    if (row >= 1 and row <= self.height) and (col >=1 and col <= self.width) then
        -- We better check if we are inside the grid
        return self.grid[row][col]
    else
        return nil
    end
end

function Grid_2D:getAdjacentCells(row, col)
    -- Returns a list of cells adjacent the ones we give
    -- REMEMBER: We index at 1 so the first row is 1, the last one is at
    -- "height", same goes for columns
    local toReturn = {}  -- We assume arrays can resize when necessary
    if (row >= 1 and row <= self.height) and (col >=1 and col <= self.width) then
        -- We better check if we are inside the grid
        if (row > 1) then
            -- We are not on the first row, we can add the cell above
            table.insert(toReturn, self.getCell(row - 1, col))
        end
        if (row < self.height) then
            -- We are not on the last row, we can add the cell below
            table.insert(toReturn, self.getCell(row + 1, col))
        end
        if (col > 1) then
            -- We are not on the first column, we can add the cell on the left
            table.insert(toReturn, self.getCell(row, col - 1))
        end
        if (col < self.width) then
            -- We are not on the last column, we can add the cell on the right
            table.insert(toReturn, self.getCell(row, col + 1))
        end
    end
    -- If the checks went well, toReturn will have
    -- a list of the adjacent cells, if not it will be empty
    return toReturn
end
