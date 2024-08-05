def clamp(value: float, min: float, max: float) -> float:
    # Clamps "value" so it is always between "min" and "max"
    if value < min:
        return min
    if value > max:
        return max
    return value
