function createNewTiles() -> Map<Tile, int>{
    // Our falling tiles list, will be used for tweening
    Map<Tile, int> falling_tiles = {};
    // We scroll each column of the board
    for (each column in matrix){
        for (each tile in column){
            if (tile == null){
                Tile new_tile = new random Tile;
                new_tile.y = - 64; // A value that is out of the board
                matrix[column][tile] = new_tile;

                // Add this tile to the falling tiles mapping
                falling_tiles[new_tile] = tile.y;
            }
        }
    }
    return falling_tiles;
}
