struct Point {
    // Rewritten as a memo
    x: u32,
    y: u32,
}

struct Rectangle {
    corner: Point,
    width: u32,
    height: u32,
    // ...
}

impl Rectangle {
    // ...
    fn from_points(topleft: &Point, bottomright: &Point) -> Self {
        // ...
    }

    fn bounding_box(vertices: &[Point]) -> Self {
        // First we create and bootstrap the variables
        let mut xmin = vertices[0].x;
        let mut xmax = vertices[0].x;
        let mut ymin = vertices[0].y;
        let mut ymax = vertices[0].y;
        // Now we iterate through all the other vertices
        for vertex in vertices {
            if vertex.x < xmin {
                xmin = vertex.x;
            }
            if vertex.x > xmax {
                xmax = vertex.x;
            }
            if vertex.y < ymin {
                ymin = vertex.y;
            }
            if vertex.y > ymax {
                ymax = vertex.y;
            }
        }
        // Now we can build the needed points for the bounding box
        let a = Point { x: xmin, y: ymin };
        let c = Point { x: xmax, y: ymax };
        // We build our bounding box
        Self::from_points(&a, &c)
    }
}
