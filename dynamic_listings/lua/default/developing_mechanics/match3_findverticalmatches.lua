function findVerticalMatches()
    local matchLength = 0
    local minMatchLength = 3
    local matches = {}
    for col = 1, #matrix do
        local column = matrix[col]
        local lastMatchingTile = nil
        for row = 1, #column do
            local currentTile = matrix[row][column].tile
            if (currentTile == lastMatchingTile) then
                matchLength = matchLength + 1
            else
                if (matchLength >= minMatchLength) then
                    -- We need to memorize all the tiles involved in the match
                    for i = row - matchLength, row do
                        local tile = matrix[i][col]
                        table.insert(matches, tile)
                    end
                else
                    -- No matches, reset the counter and set the current tile as last matching
                    matchLength = 1
                    lastMatchingTile = currentTile
                end
            end
            -- We need to account for the bottom border corner case
            if (row == #column) then
                if (matchLength >= minMatchLength) then
                    -- We need to memorize all the tiles involved in the match
                    for i = row - matchLength, row do
                        local tile = matrix[i][col]
                        table.insert(matches, tile)
                    end
                end
            end
        end
    end
    return matches
end
