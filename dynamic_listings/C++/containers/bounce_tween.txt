#include<cmath>

float bounce_tween(float t){
    // This constant will allow us to overshoot the max value by around 10%
    const float c = 1.70158;

    return 1 + (c + 1) * pow(t - 1, 3) + c * pow(t - 1, 2);
}
