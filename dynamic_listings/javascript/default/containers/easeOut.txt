function easeOut(time, duration, power){
    return 1 - (1 - (time / duration)) ** power;
}
