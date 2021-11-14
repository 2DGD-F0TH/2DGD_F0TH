function linearTween(time, begin, change, duration){
    return change * (time / duration) + begin;
}
