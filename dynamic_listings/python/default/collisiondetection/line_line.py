def lineLineCollision(
    x1: int, y1: int, x2: int, y2: int, x3: int, y3: int, x4: int, y4: int
) -> bool:
    # Let's calculate the denominator, this will allow us to avoid a
    # "divide by zero" error
    den: int = ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1))

    if den == 0:
        # The lines are parallel
        return False

    uA: float = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / den
    uB: float = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / den

    # Let's see if uA and uB tell us the lines are colliding
    if (0 <= uA <= 1) and (0 <= uB <= 1):
        return True

    # If not, they don't collide
    return False
