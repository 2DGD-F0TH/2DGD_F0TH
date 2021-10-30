#include <cmath>

struct Tile{
    int x;
    int y;
};

float euclidean_distance(Tile start, Tile goal){
    return std::sqrt(std::pow((start.x - goal.x), 2) + std::pow((start.y - goal.y), 2));
}
