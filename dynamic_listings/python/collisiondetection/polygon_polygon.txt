# ...
def polygon_polygon(p1, p2):
    # First we do a polygon vs line check for all the edges
    for i in range(len(p2.vertices)):
        j = i + 1
        if j == len(p2.vertices):
            # Wrap around in case we get to the end
            j = 0
        temp_line = Line.fromPoints(p2.vertices[i], p2.vertices[j])
        if polygon_line(p1, temp_line):
            # We have a hit
            return True
    # Now we check in case one polygon contains the other, we can just check a single vertex
    if polygon_point(p1, p2.vertices[0]) or polygon_point(p2, p1.vertices[0]):
        return True
    # None of the checks was triggered, there is no collision
    return False
