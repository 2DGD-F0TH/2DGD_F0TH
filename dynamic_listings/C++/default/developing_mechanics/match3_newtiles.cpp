#include <unordered_map>

std::unordered_map<Tile, int> createNewTiles(){
    // Our falling tiles list, will be used for tweening
    std::unordered_map<Tile, int> falling_tiles = {};
    // We scroll each column of the board
    for (Tile* column[] : matrix){
        for (Tile* tile : column){
            if (tile == nullptr){
                Tile* new_tile = Tile::create_random();
                new_tile->y = - 64; // A value that is out of the board
                matrix[column][tile] = new_tile;

                // Add this tile to the falling tiles mapping
                falling_tiles[new_tile] = tile->y;
            }
        }
    }
    return falling_tiles;
}
