def remove_matches():
    for tile in matches:
        matrix[tile.x][tile.y] = None
    matches = []


def find_falling_tiles() -> dict[Tile, int]:
    # Our falling tiles list, will be used for tweening
    falling_tiles: dict[Tile, int] = {}
    # We scroll each column of the board
    for column in matrix:
        found_space: bool = False
        space_y: int = 0
        # We scroll each row, from bottom to top
        y: int = len(column) #number of rows
        while (y > 0):
            tile: Tile = matrix[y][x]
            if found_space:
                # If the current tile is not a space, bring it to the lowest space
                if tile:
                    # Put it in the correct spot
                    matrix[space_y][column] = tile
                    tile.y = space_y

                    # Set the old position to empty
                    matrix[column][y] = None

                    # Set the tween starting position for later
                    falling_tiles[tile] = y

                    # We reset the found_space for next loop
                    found_space = False
                    # We need to re-scan this tile (it will be empty, but there may be more tiles above)
                    y = space_y

                    # Reset spaceY for next loop
                    space_y = 0
            elif not tile:
                found_space = True
                # In case we didn't find a space yet, this is the one
                if not space_y:
                    space_y = y
            # We go up one tile
            y -= 1
    return falling_tiles
