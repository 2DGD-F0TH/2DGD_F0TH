class Point{
    // Rewritten as a memo
    constructor(){
        this.x = null;
        this.y = null;
    }
}

class Rectangle{
    constructor(){
        this.corner = new Point();
        this.width = 0;
        this.height = 0;
    }

    static from_points(topleft, bottomright){
        // ...
    }
}

function bounding_box(vertices){
    // First we create and bootstrap the variables
    let xmin = vertices[0].x;
    let xmax = vertices[0].x;
    let ymin = vertices[0].y;
    let ymax = vertices[0].y;
    // Now we iterate through all the other vertices
    for (const vertex in vertices){
        if (vertex.x < xmin){
            xmin = vertex.x;
        }
        if (vertex.x > xmax){
            xmax = vertex.x;
        }
        if (vertex.y < ymin){
            ymin = vertex.y;
        }
        if (vertex.y > ymax){
            ymax = vertex.y;
        }
    }
    // Now we can build the needed points for the bounding box
    A = new Point();
    C = new Point();
    A.x = xmin;
    A.y = ymin;
    C.x = xmax;
    C.y = ymax;
    // We build our bounding box
    let boundingBox = Rectangle.from_points(A, C);
    // and return it
    return boundingBox;
}
