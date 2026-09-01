fn line_line_collision(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> bool {
    // Let's calculate the denominator, this will allow us to avoid a
    // "divide by zero" error
    let den = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

    if den == 0. {
        // The lines are parallel
        return false;
    }

    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / den;
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / den;

    // Let's see if uA and uB tell us the lines are colliding
    if (u_a >= 0. && u_a <= 1.) && (u_b >= 0. && u_b <= 1.) {
        return true;
    }

    // If not, they don't collide
    false
}
