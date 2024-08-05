#include <cmath>

struct Circle{
    // Let's define a circle class/structure
    Point center;
    int radius;
};

float distance(Point A, Point B){
    // Calculates the distance between two points
    return std::sqrt(std::pow((A.x - B.x),2) + std::pow((A.y - B.y),2));
}

bool circle_circle_collision(Circle A, Circle B){
    return distance(A.center, B.center) <= A.radius + B.radius;
}
