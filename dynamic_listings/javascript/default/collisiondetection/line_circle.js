class Point{
    // Rewritten as a memo
    constructor(){
        this.x = null;
        this.y = null;
    }

    constructor(x, y){
        this.x = x;
        this.y = y;
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

function distance(A, B){
    // Calculates the distance between two points
    return Math.sqrt((A.x - B.x) ** 2 + (A.y - B.y) ** 2);
}

function line_point_collision(line, point){
    // ...
}

function circle_point_collision(circ, point){
    // ...
}

function line_circle_collision(circle, line){
    // We check the ends first
    let collides_A = circle_point_collision(circle, line.A);
    let collides_B = circle_point_collision(circle, line.B);
    if (collides_A || collides_B){
        return true;
    }
    // We pre-calculate "u", we'll use some variables for readability
    let x1 = line.A.x;
    let x2 = line.B.x;
    let xk = circle.center.x;
    let y1 = line.A.y;
    let y2 = line.B.y;
    let yk = circle.center.y;
    let u = ((xk - x1) * (x2 - x1) + (yk - y1) * (y2 - y1))/(distance(line.A, line.B)) ** 2;
    // Now let's calculate the x and y coordinates
    let x = x1 + u * (x2 - x1);
    let y = y1 + u * (y2 - y1);
    // "Reuse": we'll use some older functions, let's create a point, with the coordinates we found
    let P = new Point(x,y);
    // Let's check if the "closest point" we found is on the line
    if ((line_point_collision(line, P)) == false){
        // If the point is outside the line, we return false, because the ends have already been checked against collisions
        return false
    }else{
        // Let's Reuse the Point/Circle Algorithm
        return circle_point_collision(circle, P);
    }
}
