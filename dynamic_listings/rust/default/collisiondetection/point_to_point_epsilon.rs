fn point_collision(a: &Point, b: &Point) -> bool {
    let epsilon = 0.0001; // Let's take a sufficiently low value
    // If both coordinates are "close enough", we trigger a collision.
    // We take the absolute value, just in case some subtractions end up being negative.
    (a.x - b.x).abs() <= epsilon && (a.y - b.y).abs() <= epsilon
}
