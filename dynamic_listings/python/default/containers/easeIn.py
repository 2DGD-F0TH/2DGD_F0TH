def ease_in(time: float, duration: float, power: float) -> float:
    return (time/duration) ** power
