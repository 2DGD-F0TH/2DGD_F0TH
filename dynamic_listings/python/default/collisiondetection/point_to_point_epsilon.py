function point_collision(A, B){
    let epsilon = 0.0001; // Let's take a sufficiently low value
    // If both coordinates are "close enough", we trigger a collision.
    // We take the absolute value, just in case some subtractions end up being negative.
    return Math.abs(A.x - B.x) <= epsilon && Math.abs(A.y - B.y) <= epsilon;
}
