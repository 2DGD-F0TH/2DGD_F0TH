def point_collision(A, B):
    """
    Detects a collision between two points

    :A: First Point
    :B: Second Point
    :returns: A boolean telling us if the two points collide

    """
    if A.x == B.x and A.y == B.y:
        return True
    else:
        return False
