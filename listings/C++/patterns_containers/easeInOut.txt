float easeInOut(const float& time, const float& duration, const float& power){
    float threshold = duration / 2;
    if (time <= threshold){
        return easeIn(time, duration, power);
    }
    return easeOut(time,duration, power);
}
