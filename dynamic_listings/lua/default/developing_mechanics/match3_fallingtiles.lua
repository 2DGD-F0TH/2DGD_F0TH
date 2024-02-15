local function removeMatches()
    for tile_idx = 1, #matches do
        matrix[tile.y][tile.x] = nil
    end
    matches = []
end

local function findFallingTiles()
    -- Our falling tiles list, will be used for tweening
    local falling_tiles = {}
    -- We scroll each column of the board
    for column_idx = 1, #matrix do
        local column = matrix[column_idx]
        local found_space = false
        local spaceY = 0
        -- We scroll each row, from bottom to top
        local y = #column
        while (y > 0) do
            local tile = matrix[y][x]
            if (found_space) then
                -- If the current tile is not a space, bring it to the lowest space
                if (tile ~= nil) then
                    -- Put it in the correct spot
                    matrix[spaceY][column] = tile
                    tile.y = spaceY

                    -- Set the old position to empty
                    matrix[column][y] = null

                    -- Set the tween starting position for later
                    falling_tiles[tile] = y

                    -- We reset the found_space for next loop
                    found_space = false
                    -- We need to re-scan this tile (it will be empty, but there may be more tiles above)
                    y = spaceY

                    -- Reset spaceY for next loop
                    spaceY = 0
                end
            elseif (tile == nil) then
                found_space = true
                -- In case we didn't find a space yet, this is the one
                if (spaceY == 0) then
                    spaceY = y
                end
            end
            -- We go up one tile
            y = y - 1
        end
    end
    return falling_tiles
end
