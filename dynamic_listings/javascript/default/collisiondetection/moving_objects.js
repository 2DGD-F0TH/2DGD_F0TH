// First, we prepare some useful functions
function dot_product(u, v){
    return (u.x * v.x) + (u.y * v.y);
}

function scale_vector(factor, v){
    return Vector2D(
        x = factor * v.x,
        y = factor * v.y
    );
}

function magnitude(v){
    return Math.sqrt(dot_product(v, v));
}

// ...
if collides(obj1, obj2){
    // Here we know that obj1 and obj2 are colliding, and we assume
    // they are moving

    // Since the "position" field is a vector, we can easily calculate "ucoll"
    let ucoll = obj2.position - obj1.position;
    // Now we calculate its relative unit vector
    let unit_ucoll = ucoll / magnitude(ucoll);
    // Let's calculate the relative velocity of the objects, since
    // the "velocity" field is a vector, that's easy
    let vrel = obj2.velocity - obj1.velocity;
    // Now we calculate s
    let s = dot_product(unit_ucoll, vrel);
    // If s > 0, we need to change the velocity of the objects
    if (s > 0){
        let factor = dot_product(s, unit_ucoll);
        obj2.velocity = scale_vector(factor, obj2.velocity);
        obj1.velocity = scale_vector(factor, obj1.velocity);
    }
    // ...
}
// ...
