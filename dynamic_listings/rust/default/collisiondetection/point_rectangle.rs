# #![allow(dead_code)]
fn point_rect_collision(
    x1: f32,
    y1: f32,
    rectx: f32,
    recty: f32,
    rectwidth: f32,
    rectheight: f32,
) -> bool {
    // We check if the point is inside the rectangle
    x1 >= rectx && x1 <= rectx + rectwidth && y1 >= recty && y1 <= recty + rectheight
}
