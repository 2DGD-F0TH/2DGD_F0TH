#include <cmath>

struct Tile{
    int x;
    int y;
};

float manhattan_distance(Tile start, Tile goal){
    return std::abs(start.x - goal.x) + std::abs(start.y - goal.y);
}
