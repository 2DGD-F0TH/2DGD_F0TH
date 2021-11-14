def pointRectCollision(x1, y1, rectx, recty, rectwidth, rectheight):
    # We check if the point is inside the rectangle
    return rectx <= x1 <= rectx + rectwidth and recty <= y1 <= recty + rectheight
