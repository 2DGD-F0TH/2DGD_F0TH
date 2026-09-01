fn point_triangle_collision(
    px: f32,
    py: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
) -> bool {
    let original_area = ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs();
    let area1 = ((x1 - px) * (y2 - py) - (x2 - px) * (y1 - py)).abs();
    let area2 = ((x2 - px) * (y3 - py) - (x3 - px) * (y2 - py)).abs();
    let area3 = ((x3 - px) * (y1 - py) - (x1 - px) * (y3 - py)).abs();
    area1 + area2 + area3 == original_area
}
