# ...
def circle_polygon_collision(poly: Polygon, circ: Circle) -> bool:
    # Case C (and partly B) are less resource-intensive than
    # a point/polygon check, so let's do them first
    for i in range(len(poly.vertices)):
        # We iterate through all the vertices
        j: int = i + 1
        # If we get to the end, we wrap around j
        if j == len(poly.vertices):
            j = 0
        temp_line: Line = Line.from_points(poly.vertices[i], poly.vertices[j])
        # In case we find a hit, we already know there is a collision
        if line_circle_collision(circ, temp_line):
            return True
        
    # Now Let's check for cases "A" and "B"
    # A: If the center is inside the polygon, we have a collision
    if polygon_point_collision(poly, circ.center):
        return True
    # B: if this rare edge-case happens, all the vertices
    # are inside the circle, so we can only check one of them
    # If any vertex is inside the circle, we have a collision, so we check the first
    if circle_point_collision(circ, poly.vertices[0]):
        return True
    # If none of the checks above returned, we don't have a collision (case D)
    return False