def lineLineCollision(
    x1: int, y1: int, x2: int, y2: int, x3: int, y3: int, x4: int, y4: int
) -> bool:
    # our previous implementation of the line/line collision detection
    pass


def pointRectCollision(
    x1: int, y1: int, rectx: int, recty: int, rectwidth: int, rectheight: int
) -> bool:
    # our previous implementation of a point/rectangle collision detection
    pass


def lineRectangleCollision(
    x1: int, y1: int, x2: int, y2: int, rectx: int, recty: int, rectwidth: int,
    rectheight: int
) -> bool:
    # If we want to test if a line is completely inside of a rect, we just need
    # to see if any of its endpoints is inside the rectangle
    if (pointRectCollision(x1, y1, rectx, recty, rectwidth, rectheight) or pointRectCollision(x2, y2, rectx, recty, rectwidth, rectheight)):
        # At least one of the ends of the segment is inside the rectangle
        return True

    # Now to test the rectangle against the line, if it's not completely inside
    left: bool = lineLineCollision(x1, y1, x2, y2, rectx, recty, rectx, recty + rectheight)
    right: bool = lineLineCollision(x1, y1, x2, y2, rectx + rectwidth, recty, rectx + rectwidth, recty + rectheight)
    top: bool = lineLineCollision(x1, y1, x2, y2, rectx, recty, rectx + rectwidth, recty)
    bottom: bool = lineLineCollision(x1, y1, x2, y2, rectx, recty + rectheight, rectx + rectwidth, recty + rectheight)

    if left or right or top or bottom:
        # We hit one of the sides, we are colliding
        return True

    # In any other case, return false
    return False
