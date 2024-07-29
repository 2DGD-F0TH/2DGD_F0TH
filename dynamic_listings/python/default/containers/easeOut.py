def ease_out(time: float, duration: float, power: float) -> float:
    return 1 - (1 - (time / duration)) ** power
