#include <cmath>

struct Circle{
    Point center;
    float radius;
};

float distance(Point A, Point B){
    // Calculates the distance between two points
    return std::sqrt(std::pow((A.x - B.x),2) + std::pow((A.y - B.y),2));
}

bool circle_point_collision(Circle A, Point B){
    return distance(A.center, B) <= A.radius;
}
