def removeMatches():
    for tile in matches:
        matrix[tile.x][tile.y] = None
    matches = []


def findFallingTiles:
    # Our falling tiles list, will be used for tweening
    falling_tiles = {}
    # We scroll each column of the board
    for (column in matrix):
        found_space = False
        spaceY = 0
        # We scroll each row, from bottom to top
        y = number of rows
        while (y > 0):
            Tile tile = matrix[y][x]
            if (found_space):
                # If the current tile is not a space, bring it to the lowest space
                if (tile is not None):
                    # Put it in the correct spot
                    matrix[spaceY][column] = tile
                    tile.y = spaceY

                    # Set the old position to empty
                    matrix[column][y] = None

                    # Set the tween starting position for later
                    falling_tiles[tile] = y

                    # We reset the found_space for next loop
                    found_space = false
                    # We need to re-scan this tile (it will be empty, but there may be more tiles above)
                    y = spaceY

                    # Reset spaceY for next loop
                    spaceY = 0
            elif (tile is None):
                found_space = True
                # In case we didn't find a space yet, this is the one
                if (spaceY == 0):
                    spaceY = y
            # We go up one tile
            y -= 1
    return falling_tiles
