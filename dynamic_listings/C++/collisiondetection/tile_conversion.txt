#include <cmath>

const int TILE_WIDTH = 32;
const int TILE_HEIGHT = 32;

int[] convert_pixels_to_tile(int x, int y){
    // Converts a point into tile coordinates
    int tile_x = std::floor(x / TILE_WIDTH);
    int tile_y = std::floor(y / TILE_HEIGHT);
    int to_return[2] = {tile_x, tile_y};
    return to_return;
}
