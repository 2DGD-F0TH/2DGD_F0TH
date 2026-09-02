# #![allow(dead_code, unused)]
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

fn distance(a: &Point, b: &Point) -> f32 {
    // Calculates the distance between two points
    ((a.x - b.x).pow(2) as f32 + (a.y - b.y).pow(2) as f32).sqrt()
}

fn line_point_collision(l: &Line, p: &Point) -> bool {
    // ...
#   todo!()
}

fn circle_point_collision(c: &Circle, p: &Point) -> bool {
    // ...
#   todo!()
}

fn line_circle_collision(circle: &Circle, line: &Line) -> bool {
    // We check the ends first
    let collides_a = circle_point_collision(circle, &line.a);
    let collides_b = circle_point_collision(circle, &line.b);
    if collides_a || collides_b {
        return true;
    }
    // ...
    // We pre-calculate "u", we'll use some variables for readability
    let x1 = line.a.x as f32;
    let x2 = line.b.x as f32;
    let xk = circle.center.x as f32;
    let y1 = line.a.y as f32;
    let y2 = line.b.y as f32;
    let yk = circle.center.y as f32;
    let u = ((xk - x1) * (x2 - x1) + (yk - y1) * (y2 - y1)) / distance(&line.a, &line.b).powi(2);
    // Now let's calculate the x and y coordinates
    let x = x1 + u * (x2 - x1);
    let y = y1 + u * (y2 - y1);
    // "Reuse", we'll use some older functions, let's create a point, with the coordinates we found
    let p = Point {
        x: x as u32,
        y: y as u32,
    };
    // Let's check if the "closest point" we found is on the line
    if !line_point_collision(line, &p) {
        // If the point is outside the line, we return false, because the ends have already been checked against collisions
        false
    } else {
        // Let's Reuse the Point/Circle Algorithm
        circle_point_collision(circle, &p)
    }
}
