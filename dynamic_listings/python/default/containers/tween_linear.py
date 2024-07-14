def linear_tween(time: float, begin: float, change: float, duration: float) -> float:
    return change * (time / duration) + begin
