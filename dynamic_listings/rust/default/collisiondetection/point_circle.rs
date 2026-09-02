# #![allow(dead_code)]
# struct Point {
#     x: f32,
#     y: f32,
# }
struct Circle {
    center: Point,
    radius: f32,
}

fn distance(a: &Point, b: &Point) -> f32 {
    // Calculates the distance between two points
    ((a.x - b.x).powi(2) as f32 + (a.y - b.y).powi(2) as f32).sqrt()
}

fn circle_point_collision(a: &Circle, b: &Point) -> bool {
    distance(&a.center, b) <= a.radius
}
