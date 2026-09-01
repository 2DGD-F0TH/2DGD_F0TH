// ...
fn line_polygon(line: &Line, poly: &Polygon) -> bool {
    // First of all, let's check if either of the line ends are inside the polygon
    // This covers cases AB and CD
    if polygon_point(&poly, &line.a) {
        // One of the ends is inside the polygon, we have a hit
        return true;
    }
    if polygon_point(&poly, &line.b) {
        // One of the ends is inside the polygon, we have a hit
        return true;
    }
    // Now we check for case EF
    for i in 0..poly.vertices.len() {
        // We iterate through all the vertices
        let mut j = i + 1;
        // If we get to the end, we wrap around j
        if j == poly.vertices.len() {
            j = 0;
        }
        let temp_line = Line::from_points(&poly.vertices[i], &poly.vertices[j]);
        if line_line_collision(&temp_line, line) {
            return true;
        }
    }
    // If none of the previous checks was triggered, we don't have a collision
    false
}
