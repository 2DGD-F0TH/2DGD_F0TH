#include <cmath>

float easeIn(const float& time, const float& duration, const float& power){
    return std::pow((time/duration), power);
}
