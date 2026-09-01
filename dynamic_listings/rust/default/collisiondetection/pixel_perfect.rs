struct Color {
    color_data: u32,
}

impl Color {
    fn is_white(&self) -> bool {
        //...
    }
}

struct Bitmask {
    data: Vec<Color>,
}

impl Bitmask {
    fn color(&self, x: u32, y: u32) -> Color {
        // ...
    }
}

struct Sprite {
    bitmask: Bitmask,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn pixel_perfect_collision(a: &Sprite, b: &Sprite) -> bool {
    // Calculate the intersecting rectangle to limit checks
    let x1 = u32::max(a.x, b.x);
    let x2 = u32::min(a.x + a.width, b.x + b.width);

    let y1 = u32::max(a.y, b.y);
    let y2 = u32::min(a.y + a.height, b.y + b.height);

    // For each pixel in the intersecting rectangle, let's check
    for y in y1..y2 {
        for x in x1..x2 {
            // We're working in the intersecting triangle, so we'll need to
            // rework our coordinates
            let a = a.bitmask.color(x - a.x, y - a.y);
            let b = b.bitmask.color(x - b.x, y - b.y);
            if a.is_white() && b.is_white() {
                return true;
            }
        }
    }

    // If no collision is occurred by the end of the checking, we're safe
    false
}
