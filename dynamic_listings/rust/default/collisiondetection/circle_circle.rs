struct Circle {
    // Let's define a circle class/structure
    center: Point,
    radius: f32,
}

fn distance(a: &Point, b: &Point) -> f32 {
    // Calculates the distance between two points
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn circle_circle_collision(a: &Circle, b: &Circle) -> bool {
    distance(&a.center, &b.center) <= a.radius + b.radius
}
