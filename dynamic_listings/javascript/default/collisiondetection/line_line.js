function lineLineCollision(x1, y1, x2, y2, x3, y3, x4, y4){
    // Let's calculate the denominator, this will allow us to avoid a
    // "divide by zero" error
    let den = ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

    if (den == 0){
        // The lines are parallel
        return false;
    }

    let uA = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / den;
    let uB = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / den;

    // Let's see if uA and uB tell us the lines are colliding
    if ((uA >= 0 && uA <= 1) && (uB >= 0 && uB <= 1)){
        return true;
    }

    // If not, they don't collide
    return false;
}
