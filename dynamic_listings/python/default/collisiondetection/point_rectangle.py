def point_rect_collision(x1: float, y1: float, rect_x: float, rect_y: float, rect_width: float, rect_height: float):
    # We check if the point is inside the rectangle
    return rect_x <= x1 <= rect_x + rect_width and rect_y <= y1 <= rect_y + rect_height
