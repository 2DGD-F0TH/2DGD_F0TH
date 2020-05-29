def easeInOut(time, duration, power):
    threshold = duration / 2
    if time <= threshold:
        return easeIn(time, duration, power)
    return easeOut(time, duration, power)
