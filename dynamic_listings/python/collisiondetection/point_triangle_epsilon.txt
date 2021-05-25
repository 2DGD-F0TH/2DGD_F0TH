def point_triangle_collision(px, py, x1, y1, x2, y2, x3, y3):
    # We accept anything that is closer than 1/1000th of unit
    EPSILON = 0.0001
    original_area = abs((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1))
    area1 = abs((x1-px)*(y2-py) - (x2-px)*(y1-py))
    area2 = abs((x2-px)*(y3-py) - (x3-px)*(y2-py))
    area3 = abs((x3-px)*(y1-py) - (x1-px)*(y3-py))
    if - EPSILON < (area1 + area2 + area3 - original_area) < EPSILON:
        return True
    return False
