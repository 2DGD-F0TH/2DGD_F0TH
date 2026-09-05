# #![allow(dead_code)]
# struct Point;
# struct Polygon {
#     vertices: Vec<Point>,
# }
# struct Line;
# impl Line {
#     fn from_points(_: &Point, _: &Point) -> Self { todo!() }
# }
# fn polygon_point(_: &Polygon, _: &Point) -> bool { todo!() }
# fn polygon_line(_: &Polygon, _: &Line) -> bool { todo!() }
// ...
fn polygon_polygon(p1: &Polygon, p2: &Polygon) -> bool {
    // First we do a polygon vs line check for all the edges
    for i in 0..p2.vertices.len() {
        let mut j = i + 1;
        if j == p2.vertices.len() {
            // Wrap around in case we get to the end
            j = 0;
        }
        let temp_line = Line::from_points(&p2.vertices[i], &p2.vertices[j]);
        if polygon_line(p1, &temp_line) {
            // We have a hit
            return true;
        }
    }
    // Now we check in case one polygon contains the other, we can just check a single vertex
    if polygon_point(p1, &p2.vertices[0]) || polygon_point(p2, &p1.vertices[0]) {
        return true;
    }
    // None of the checks was triggered, there is no collision
    false
}
