# #![allow(dead_code)]
struct Point {
    x: u32,
    y: u32,
}

struct Line {
    a: Point,
    b: Point,
}

struct Circle {
    center: Point,
    radius: u32,
}

# fn circle_point_collision(_: &Circle, _: &Point) -> bool { todo!() }
fn line_circle_collision(circle: &Circle, line: &Line) -> bool {
    let collides_a = circle_point_collision(circle, &line.a);
    let collides_b = circle_point_collision(circle, &line.b);
    if collides_a || collides_b {
        return true;
    }
    // ...
#   todo!()
}
