# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
#
# struct Object {
#     position: Vector2D,
#     velocity: Vector2D,
# }
fn dot_product(u: &Vector2D, v: &Vector2D) -> f32 {
    u.x * v.x + u.y * v.y
}

fn scale_vector(factor: f32, v: &Vector2D) -> Vector2D {
    Vector2D {
        x: factor * v.x,
        y: factor * v.y,
    }
}

fn magnitude(v: &Vector2D) -> f32 {
    dot_product(v, v).sqrt()
}

// ...
fn collides(obj1: &mut Object, obj2: &mut Object) {
    // Here we know that obj1 and obj2 are colliding, and we assume
    // they are moving

    // Since the "position" field is a vector, we can easily calculate "ucoll"
    let ucoll = obj2.position - obj1.position;
    // Now we calculate its relative unit vector
    let unit_ucoll = ucoll / magnitude(&ucoll);
    // Let's calculate the relative velocity of the objects, since
    // the "velocity" field is a vector, that's easy
    let vrel = obj2.velocity - obj1.velocity;
    // Now we calculate s
    let s = dot_product(&unit_ucoll, &vrel);
    // If s < 0, we need to change the velocity of the objects
    if s < 0. {
        obj2.velocity += scale_vector(s, &unit_ucoll);
        obj1.velocity += scale_vector(s, &unit_ucoll);
    }
    // ...
}
// ...
