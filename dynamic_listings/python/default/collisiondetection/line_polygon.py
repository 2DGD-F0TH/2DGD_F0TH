# ...
def line_polygon(line: Line, poly: Polygon) -> bool:
    # First of all, let's check if either of the line ends are inside the polygon
    # This covers cases AB and CD
    if polygon_point(poly, line.A):
        # One of the ends is inside the polygon, we have a hit
        return True
    if polygon_point(poly, line.B):
        # One of the ends is inside the polygon, we have a hit
        return True
    # Now we check for case EF
    for i in enumerate(poly.vertices):
        # We iterate through all the vertices
        j: int = i + 1
        # If we get to the end, we wrap around j
        if j == len(poly.vertices):
            j = 0
        temp_line: Line = Line.fromPoints(poly.vertices[i], poly.vertices[j])
        if line_line_collsion(temp_line, line):
            return True
    if line_line_collision(temp_line, line):
        return True
    # If none of the previous checks was triggered, we don't have a collision
    return False
