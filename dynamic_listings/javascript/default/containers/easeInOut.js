function easeInOut(time, duration, power){
    let threshold = duration / 2;
    if (time <= threshold){
        return easeIn(time, duration, power);
    }else{
        return easeOut(time,duration, power);
    }
}
