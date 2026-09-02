# #![allow(dead_code)]
#
# struct Point {
#     x: u32,
#     y: u32,
# }
# struct Circle {
#     center: Point,
# }
# struct Polygon {
#     vertices: Vec<Point>,
# }
# struct Line {
# }
# impl Line {
#     fn from_points(_: &Point, _: &Point) -> Self { todo!() }
# }
# fn line_circle_collision(_: &Circle, _: &Line) -> bool { todo!() }
# fn polygon_point(_: &Polygon, _: &Point) -> bool { todo!() }
# fn circle_point_collision(_: &Circle, _: &Point) -> bool { todo!() }
// ...
fn circle_polygon(poly: &Polygon, circ: &Circle) -> bool {
    // Case C (and partly B) are less resource-intensive than
    // a point/polygon check, so let's do them first
    for i in 0..poly.vertices.len() {
        // We iterate through all the vertices
        let mut j = i + 1;
        // If we get to the end, we wrap around j
        if j == poly.vertices.len() {
            j = 0;
        }
        let temp_line = Line::from_points(&poly.vertices[i], &poly.vertices[j]);
        // In case we find a hit, we already know there is a collision
        if line_circle_collision(circ, &temp_line) {
            return true;
        }
    }
    // Now Let's check for cases "A" and "B"
    if polygon_point(poly, &circ.center) {
        // If the center is inside the polygon, we have a collision
        return true;
    }
    // Now let's check for the rare edge-case: if this case happens, all the vertices
    // are inside the circle, so we can only check one of them
    if circle_point_collision(circ, &poly.vertices[0]) {
        // If any vertex is inside the circle, we have a collision, so we check the first
        return true;
    }
    // If none of the checks above returned, we don't have a collision (case D)
    false
}
