# #![allow(dead_code)]
# struct Point;
# struct Triangle;
# struct Rectangle;
# struct Polygon {
#     vertices: Vec<Point>,
# }
# impl Polygon {
#     fn calculate_bounding_box(&self) -> Rectangle { todo!() }
#     fn do_fanning(&self) -> Vec<Triangle> { todo!() }
# }
# fn point_rectangle(_: &Rectangle, _: &Point) -> bool { todo!() }
# fn point_triangle(_: &Triangle, _: &Point) -> bool { todo!() }
// ...
fn polygon_point(poly: &Polygon, point: &Point) -> bool {
    // First of all, we get the polygon's bounding box
    let bounding_box = poly.calculate_bounding_box();
    // Then we do a simple point vs. rectangle check
    if !point_rectangle(&bounding_box, point) {
        // We are not even in the bounding box, we can't collide
        return false;
    }
    // If instead we are in the bounding box, we need to get the "fan triangulation"
    let triangles = poly.do_fanning();
    // Now we check, for each triangle, if the point collides
    for triangle in &triangles {
        if point_triangle(triangle, &point) {
            // We found the "slice" of the polygon that the point collides with
            return true;
        }
    }
    // If we pass all triangles without a hit, we are in the bounding box
    // but outside the polygon, that's the worst case, and we are not colliding
    false
}
