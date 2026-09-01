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

struct Circle {
    center: Point,
    radius: u32,
}

fn circle_rectangle_collision(circ: &Circle, rect: &Rectangle) -> bool {
    // Detects a collision between a circle and a rectangle

    // These variables are used as the coordinates we should test against
    // They are temporarily set to the circle center's coordinates for a reason we'll see soon
    let mut tx = circ.center.x;
    let mut ty = circ.center.y;

    // Let's detect which edge to test against on the x axis
    if circ.center.x < rect.corner.x {
        // We're at the left of the rectangle, test against the left side
        tx = rect.corner.x;
    } else if circ.center.x > rect.corner.x + rect.width {
        // We're at the right of the rectangle, test against the right side
        tx = rect.corner.y + rect.width;
    }

    // Same thing on the vertical axis
    if circ.center.y < rect.corner.y {
        // We're above the rectangle, test against the top side
        ty = rect.corner.y;
    } else if circ.center.y > rect.corner.y + rect.height {
        // We're below the rectangle, test against the bottom side
        ty = rect.corner.y + rect.height;
    }

    // Let's get the distance between the testing coordinates and the circle center
    let distance_x = circ.center.x - tx;
    let distance_y = circ.center.y - ty;
    let distance = (distance_x.pow(2) + distance_y.pow(2)).isqrt();

    // Note that if the center of the circle is inside the rectangle, the testing coordinates will be the circle's center itself, thus the next conditional will always return true

    if distance <= circ.radius {
        return true;
    }

    // Default to false in case no collision occurs
    false
}
