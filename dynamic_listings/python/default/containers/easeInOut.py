def ease_in_out(time: float, duration: float, power: float) -> float:
    threshold: float = duration / 2
    if time <= threshold:
        return easeIn(time, duration, power)
    return easeOut(time, duration, power)
