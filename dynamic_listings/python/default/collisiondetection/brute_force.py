def is_collision(A: Circle, B: Circle) -> bool:
    """
    Defines how two items collide
    (being circles, this could be a difference of radii)

    :A: First object
    :B: Second Object
    :returns: Boolean telling if the items collide

    """
    ...


items: list[Circle] = [circle1, circle2, circle3, ...]
colliding_items: list[tuple[Circle]] = []

for A in items:
    for B in items:
        if A == B:
            continue
            # We avoid checking if an item collides with itself,
            # for obvious reasons
        if is_collision(A, B):
            colliding_items.append((A, B))
