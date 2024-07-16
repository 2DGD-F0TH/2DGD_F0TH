def point_triangle_collision(px: float, py: float, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float) -> bool:
    # We accept anything that is closer than 1/1000th of unit
    EPSILON = 0.0001
    original_area: float = abs((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1))
    area1: float = abs((x1-px)*(y2-py) - (x2-px)*(y1-py))
    area2: float = abs((x2-px)*(y3-py) - (x3-px)*(y2-py))
    area3: float = abs((x3-px)*(y1-py) - (x1-px)*(y3-py))
    return -EPSILON < (area1 + area2 + area3 - original_area) < EPSILON
