# #![allow(dead_code, unused)]
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
    // our previous implementation of the line/line collision detection
# todo!()
}

fn point_rect_collision(
    x1: f32,
    y1: f32,
    rectx: f32,
    recty: f32,
    rectwidth: f32,
    rectheight: f32,
) -> bool {
    // our previous implementation of a point/rectangle collision detection
# todo!()
}

fn line_rectangle_collision(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    rectx: f32,
    recty: f32,
    rectwidth: f32,
    rectheight: f32,
) -> bool {
    // If we want to test if a line is completely inside of a rect, we just need
    // to see if any of its endpoints is inside the rectangle
    if point_rect_collision(x1, y1, rectx, recty, rectwidth, rectheight)
        || point_rect_collision(x2, y2, rectx, recty, rectwidth, rectheight)
    {
        // At least one of the ends of the segment is inside the rectangle
        return true;
    }

    // Now to test the rectangle against the line, if it's not completely inside
    let left = line_line_collision(x1, y1, x2, y2, rectx, recty, rectx, recty + rectheight);
    let right = line_line_collision(
        x1,
        y1,
        x2,
        y2,
        rectx + rectwidth,
        recty,
        rectx + rectwidth,
        recty + rectheight,
    );
    let top = line_line_collision(x1, y1, x2, y2, rectx, recty, rectx + rectwidth, recty);
    let bottom = line_line_collision(
        x1,
        y1,
        x2,
        y2,
        rectx,
        recty + rectheight,
        rectx + rectwidth,
        recty + rectheight,
    );

    if left || right || top || bottom {
        // We hit one of the sides, we are colliding
        return true;
    }

    // In any other case, return false
    return false;
}
