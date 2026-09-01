struct Point {
    x: u32,
    y: u32,
}

struct Line {
    a: Point,
    b: Point,
}

fn distance(a: &Point, b: &Point) -> f32 {
    // Calculates the distance between two points
    ((a.x - b.x).pow(2) as f32 + (a.y - b.y).pow(2) as f32).sqrt()
}

fn line_point_collision(pt: &Point, ln: &Line) -> bool {
    // First, let's calculate the length of the line
    let length = distance(&ln.a, &ln.b);
    // Now let's calculate the distance between the point pt
    // and the point "A" of the line
    let pt_a = distance(&ln.a, pt);
    // Same Goes for the distance between pt and "B"
    let pt_b = distance(&ln.b, pt);
    // Now for the detection
    pt_a + pt_b == length
}
