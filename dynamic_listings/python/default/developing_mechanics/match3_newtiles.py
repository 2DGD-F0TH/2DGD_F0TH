def create_new_tiles():
    # Our falling tiles list, will be used for tweening
    falling_tiles: dict[Tile, int] = {}
    # We scroll each column of the board
    for column in matrix:
        for tile in column:
            if tile is None:
                new_tile: Tile = Tile.random()
                new_tile.y = - 64   # A value that is out of the board
                matrix[column][tile] = new_tile

                # Add this tile to the falling tiles mapping
                falling_tiles[new_tile] = tile.y
    return falling_tiles
