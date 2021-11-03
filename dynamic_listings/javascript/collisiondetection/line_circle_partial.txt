class Point{
    // Rewritten as a memo
    constructor(){
        this.x = null;
        this.y = null;
    }
}

class Line{
    constructor(){
        this.A = new Point();
        this.B = new Point();
    }
}

class Circle{
    // Let's define a circle class/structure
    constructor(){
        this.center = new Point();
        this.radius = 0;
    }
}

// ...

function line_circle_collision(circle, line){
    let collides_A = circle_point_collision(circle, line.A);
    let collides_B = circle_point_collision(circle, line.B);
    if (collides_A || collides_B){
        return true;
    }
    // ...
}
