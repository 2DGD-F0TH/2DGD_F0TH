def point_point_collision(A: Point, B: Point) -> bool:
    """
    Detects a collision between two points

    :A: First Point
    :B: Second Point
    :returns: A boolean telling us if the two points collide

    """
    return A.x == B.x and A.y == B.y
