// ...
bool line_polygon(Line line, Polygon poly){
    // First of all, let's check if either of the line ends are inside the polygon
    // This covers cases AB and CD
    if (polygon_point(poly, line.A)){
        // One of the ends is inside the polygon, we have a hit
        return true;
    }
    if (polygon_point(poly, line.B)){
        // One of the ends is inside the polygon, we have a hit
        return true;
    }
    // Now we check for case EF
    for (int i = 0; i < poly.vertices.length(); i++){
        // We iterate through all the vertices
        j = i + 1;
        // If we get to the end, we wrap around j
        if (j == poly.vertices.length()){
            j = 0;
        }
        Line temp_line = Line.fromPoints(poly.vertices[i], poly.vertices[j]);
        if (line_line_collsion(temp_line, line)){
            return true;
        }
    }
    // If none of the previous checks was triggered, we don't have a collision
    return false;
}
