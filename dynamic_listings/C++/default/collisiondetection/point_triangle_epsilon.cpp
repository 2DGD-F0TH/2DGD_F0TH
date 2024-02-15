#include <cmath>

bool point_triangle_collision(float px, float py, float x1, float y1, float x2, float y2, float x3, float y3){
    // We accept anything that is closer than 1/1000th of unit
    const float epsilon = 0.0001;
    float original_area = std::abs((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1));
    float area1 = std::abs((x1-px)*(y2-py) - (x2-px)*(y1-py));
    float area2 = std::abs((x2-px)*(y3-py) - (x3-px)*(y2-py));
    float area3 = std::abs((x3-px)*(y1-py) - (x1-px)*(y3-py));
    if (std::abs(area1 + area2 + area3 - original_area) < epsilon){
        return true;
    }else{
        return false;
    }
}
