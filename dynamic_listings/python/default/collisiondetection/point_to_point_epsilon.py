def point_point_collision(A: Point, B: Point) -> bool:
    epsilon = 0.0001 #Let's take a sufficiently low value
    # If both coordinates are "close enough", we trigger a collision.
    # We take the absolute value, just in case some subtractions end up being negative.
    return abs(A.x - B.x) <= epsilon and abs(A.y - B.y) <= epsilon