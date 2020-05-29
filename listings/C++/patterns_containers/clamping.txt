float clamp(const float& value, const float& min, const float& max){
    // Clamps "value" so it is always between "min" and "max"
    if (value < min){
        return min;
    }
    if (value > max){
        return max;
    }
    return value;
}
