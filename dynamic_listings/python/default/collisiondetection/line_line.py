def line_line_collision(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float, x4: float, y4: float) -> bool:
    # Let's calculate the denominator, this will allow us to avoid a
    # "divide by zero" error
    den = ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1))

    if den == 0:
        # The lines are parallel
        return False

    uA = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / den
    uB = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / den

    # Let's see if uA and uB tell us the lines are colliding
    is_colliding = 0 <= uA <= 1 and 0 <= uB <= 1
    # If not, they don't collide
    return is_colliding