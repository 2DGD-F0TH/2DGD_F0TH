# #![allow(dead_code)]
#
# struct Point {
#     x: f32,
#     y: f32,
# }
fn point_collision(a: &Point, b: &Point) -> bool {
    a.x == b.x && a.y == b.y
}
