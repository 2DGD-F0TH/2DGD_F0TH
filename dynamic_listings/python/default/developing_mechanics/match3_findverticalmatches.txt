def findVerticalMatches():
    matchLength = 0
    minMatchLength = 3
    for column in matrix:
        lastMatchingTile = None
        for row in column:
            currentTile = matrix[row][column].tile
            if currentTile == lastMatchingTile:
                matchLength += 1
            else:
                if matchLength >= minMatchLength:
                    # We need to memorize all the tiles involved in the match
                    for r in range(row-matchLength, column+1):
                        tile = matrix[r][column]
                        memorize(tile)
                else:
                    # No matches, reset the counter and set the current tile as last matching
                    matchLength = 1
                    lastMatchingTile = currentTile
            # We need to account for the bottom border corner case
            if row == len(matrix[column]):
                if matchLength >= minMatchLength:
                    # We need to memorize all the tiles involved in the match
                    for r in range(row-matchLength, column+1):
                        tile = matrix[r][column]
                        memorize(tile)
