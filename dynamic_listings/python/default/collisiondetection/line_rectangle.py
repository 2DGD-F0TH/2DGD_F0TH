def line_line_collision(x1: float, y1: float, x2: float, y2: float, x3: float, y3: float, x4: float, y4: float) -> bool:
    # our previous implementation of the line/line collision detection
    ...


def point_rect_collision(x1, y1, rectx, recty, rectwidth, rectheight) -> bool:
    # our previous implementation of a point/rectangle collision detection
    ...


def line_rectangle_collision(x1: float, y1: float, x2: float, y2: float, rectx: float, recty: float, rectwidth: float, rectheight: float) -> bool:
    # If we want to test if a line is completely inside of a rect, we just need
    # to see if any of its endpoints is inside the rectangle
    if point_rect_collision(x1, y1, rectx, recty, rectwidth, rectheight) or point_rect_collision(x2, y2, rectx, recty, rectwidth, rectheight):
        # At least one of the ends of the segment is inside the rectangle
        return True

    # Now to test the rectangle against the line, if it's not completely inside
    left = line_line_collision(x1, y1, x2, y2, rectx, recty, rectx, recty + rectheight)
    right = line_line_collision(x1, y1, x2, y2, rectx + rectwidth, recty, rectx + rectwidth, recty + rectheight)
    top = line_line_collision(x1, y1, x2, y2, rectx, recty, rectx + rectwidth, recty)
    bottom = line_line_collision(x1, y1, x2, y2, rectx, recty + rectheight, rectx + rectwidth, recty + rectheight)

    is_colliding = left or right or top or bottom
    # If we hit one of the sides, we are colliding
    # In any other case, return false
    return is_colliding