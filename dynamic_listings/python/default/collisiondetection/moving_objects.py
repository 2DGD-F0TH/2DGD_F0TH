from math import sqrt


# First, we prepare some useful functions
def dot_product(u: Vector2, v: Vector2) -> float:
    return (u.x * v.x) + (u.y * v.y)


def scale_vector(factor: float, v: Vector2) -> Vector2:
    return Vector2D(
        x=factor * v.x,
        y=factor * v.y
    )


def magnitude(v: Vector2) -> float:
    return sqrt(dot_product(v, v))


# ...
if collides(obj1, obj2):
    # Here we know that obj1 and obj2 are colliding, and we assume
    # they are moving

    # Since the "position" field is a vector, we can easily calculate "ucoll"
    ucoll: Vector2 = obj2.position - obj1.position
    # Now we calculate its relative unit vector
    unit_ucoll: Vector2 = ucoll / magnitude(ucoll)
    # Let's calculate the relative velocity of the objects, since
    # the "velocity" field is a vector, that's easy
    vrel: Vector2 = obj2.velocity - obj1.velocity
    # Now we calculate s
    s: float = dot_product(unit_ucoll, vrel)
    # If s > 0, we need to change the velocity of the objects
    if s > 0:
        factor: float = dot_product(s, unit_ucoll)
        obj2.velocity = scale_vector(factor, obj2.velocity)
        obj1.velocity = scale_vector(factor, obj1.velocity)

    # ...

# ...
