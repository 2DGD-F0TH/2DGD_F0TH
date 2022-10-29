#include <map>

void removeMatches(){
    for (Tile tile: matches){
        matrix[tile.y][tile.x] = nullptr;
    }
    matches = {};
}

std::map<Tile, int> findFallingTiles(){
    // Our falling tiles list, will be used for tweening
    std::map<Tile, int> falling_tiles = {};
    // We scroll each column of the board
    int colsize = (sizeof matrix / sizeof matrix[0]);
    for (int x = 0; x < colsize; ++x){
        bool found_space = false;
        int spaceY = 0;
        // We scroll each row, from bottom to top
        int y = (sizeof x / sizeof matrix[x][0]);
        while (y > 0){
            Tile tile = matrix[y][x];
            if (found_space){
                // If the current tile is not a space, bring it to the lowest space
                if (tile != nullptr){
                    // Put it in the correct spot
                    matrix[spaceY][x] = tile;
                    tile.y = spaceY;

                    // Set the old position to empty
                    matrix[x][y] = nullptr;

                    // Set the tween starting position for later
                    falling_tiles[tile] = y;

                    // We reset the found_space for next loop
                    found_space = false;
                    // We need to re-scan this tile (it will be empty, but there may be more tiles above)
                    y = spaceY;

                    // Reset spaceY for next loop
                    spaceY = 0;
                }
            }else if (tile == nullptr){
                found_space = true;
                // In case we didn't find a space yet, this is the one
                if (spaceY == 0){
                    spaceY = y;
                }
            }
            // We go up one tile
            y = y - 1;
        }
    }
    return falling_tiles;
}
