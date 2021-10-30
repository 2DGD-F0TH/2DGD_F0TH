#include <cmath>
#include <vector>

const int TILE_WIDTH = 32;
const int TILE_HEIGHT = 32;

struct Rectangle{
    // A rectangle will represent the player
    Point corner;
    int width;
    int height;
};

int[] convert_pixels_to_tile(int x, int y){
    // Converts a point into tile coordinates
    int tile_x = std::floor(x / TILE_WIDTH);
    int tile_y = std::floor(y / TILE_HEIGHT);
    int to_return[2] = {tile_x, tile_y};
    return to_return;
}

// We assume the player is falling, so no check will be shown here
std::vector<Point> points_to_check = {
    Point(player.corner.x, player.corner.y + player.height),
    Point(player.corner.x + player.width, player.corner.y + player.height)
}

for (Point point: points_to_check){
    int[] detected_tile_coordinates = convert_pixels_to_tile(point.x, point.y);
    Tile* detected_tile = get_tile(detected_tile_coordinates);
    if (AABB(player, detected_tile->rectangle)){
        // React to the collision
    }
}
