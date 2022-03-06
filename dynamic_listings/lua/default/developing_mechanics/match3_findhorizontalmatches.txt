function findHorizontalMatches()
    local matchLength = 0
    local minMatchLength = 3
    local matches = {}
    for row_idx = 1, #matrix do
        local row = matrix[row_idx]
        local lastMatchingTile = nil
        for col = 1, #row do
            local currentTile = matrix[row_idx][col].tile
            if (currentTile == lastMatchingTile) then
                matchLength = matchLength + 1
            else
                if (matchLength >= minMatchLength) then
                    -- We need to memorize all the tiles involved in the match
                    for i = col - matchLength, col do
                        local tile = matrix[row_idx][i]
                        table.insert(matches, tile)
                    end
                else
                    -- No matches, reset the counter and set the current tile as last matching
                    matchLength = 1
                    lastMatchingTile = currentTile
                end
            end
            -- We need to account for the right-hand border corner case
            if (col == #row) then
                if (matchLength >= minMatchLength) then
                    -- We need to memorize all the tiles involved in the match
                    for i = col - matchLength, col do
                        local tile = matrix[row_idx][i]
                        table.insert(matches, tile)
                    end
                end
            end
        end
    end
    return matches
end
