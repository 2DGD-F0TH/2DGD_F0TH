def lineLineCollision(x1, y1, x2, y2, x3, y3, x4, y4):
    # Let's calculate the denominator, this will allow us to avoid a
    # "divide by zero" error
    den = ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1))

    if den == 0:
        # The lines are parallel
        return False

    uA = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / den
    uB = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / den

    # Let's see if uA and uB tell us the lines are colliding
    if (uA >= 0 and uA <= 1) and (uB >= 0 and uB <= 1):
        return True

    # If not, they don't collide
    return False
