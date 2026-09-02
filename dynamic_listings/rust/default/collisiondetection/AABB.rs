# #![allow(dead_code)]
struct Point {
    // Rewritten as a memo
    x: u32,
    y: u32,
}

struct Rectangle {
    corner: Point,
    width: u32,
    height: u32,
}

fn rect_rect_collision(a: &Rectangle, b: &Rectangle) -> bool {
    a.corner.x < b.corner.x + b.width
        && a.corner.x + a.width > b.corner.x
        && a.corner.y < b.corner.y + b.height
        && a.corner.y + a.height > a.corner.y
}
