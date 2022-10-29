local function createNewTiles()
    -- Our falling tiles list, will be used for tweening
    local falling_tiles = {}
    -- We scroll each column of the board
    for column_idx = 1, #matrix do
        column = matrix[column_idx]
        for row_idx = 1, #column do
            tile = column[row_idx]
            if (tile == nil) then
                local new_tile = Tile:random()
                new_tile.y = - 64 -- A value that is out of the board
                matrix[column][tile] = new_tile

                -- Add this tile to the falling tiles mapping
                falling_tiles[new_tile] = tile.y
            end
        end
    end
    return falling_tiles
end
