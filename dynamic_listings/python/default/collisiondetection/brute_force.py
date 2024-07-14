def is_collision(A: Circle, B: Circle) -> bool:
    """
    Defines how two items collide
    (being circles, this could be a difference of radii)

    :A: First object
    :B: Second Object
    :returns: Boolean telling if the items collide

    """
    pass


items: list[int] = [1, 2, 3, 4, 5, 6, 7]
colliding_items: list[tuple[Circle]] = []

for A in items:
    for B in items:
        if A is not B:
            # We avoid checking if an item collides with itself,
            # for obvious reasons
            if is_collision(A, B):
                colliding_items.append((A, B))
