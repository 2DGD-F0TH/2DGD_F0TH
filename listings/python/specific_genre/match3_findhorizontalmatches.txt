def findHorizontalMatches():
    matchLength = 0
    minMatchLength = 3
    for row in matrix:
        lastMatchingTile = None
        for column in row:
            currentTile = matrix[row][column].tile
            if currentTile == lastMatchingTile:
                matchLength += 1
            else:
                if matchLength >= minMatchLength:
                    # We need to memorize all the tiles involved in the match
                    for tile in matrix[row][column-matchLength:column+1]:
                        memorize(tile)
                else:
                    # No matches, reset the counter and set the current tile as last matching
                    matchLength = 1
                    lastMatchingTile = currentTile
            # We need to account for the right-hand border corner case
            if column == len(matrix[row]):
                if matchLength >= minMatchLength:
                    # We need to memorize all the tiles involved in the match
                    for tile in matrix[row][column-matchLength:column+1]:
                        memorize(tile)
