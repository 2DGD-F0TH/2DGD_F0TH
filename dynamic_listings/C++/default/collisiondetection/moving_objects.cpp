#include <cmath>
// First, we prepare some useful functions
float dot_product(Vector2D u, Vector2D v){
    return (u.x * v.x) + (u.y * v.y);
}

Vector2D scale_vector(float factor, Vector2D v){
    return Vector2D(
        x = factor * v.x,
        y = factor * v.y
    );
}

float magnitude(Vector2D v){
    return sqrt(dot_product(v, v));
}

// ...
if (collides(obj1, obj2)){
    // Here we know that obj1 and obj2 are colliding, and we assume
    // they are moving

    // Since the "position" field is a vector, we can easily calculate "ucoll"
    Vector2D ucoll = obj2.position - obj1.position;
    // Now we calculate its relative unit vector
    Vector2D unit_ucoll = ucoll / magnitude(ucoll);
    // Let's calculate the relative velocity of the objects, since
    // the "velocity" field is a vector, that's easy
    Vector2D vrel = obj2.velocity - obj1.velocity;
    // Now we calculate s
    float s = dot_product(unit_ucoll, vrel);
    // If s > 0, we need to change the velocity of the objects
    if (s > 0){
        float factor = dot_product(s, unit_ucoll);
        obj2.velocity = scale_vector(factor, obj2.velocity);
        obj1.velocity = scale_vector(factor, obj1.velocity);
    }
    // ...
}
// ...
