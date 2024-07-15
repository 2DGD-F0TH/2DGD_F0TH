def is_collision(A: Item, B: Item) -> bool:
    """
    Defines how two items collide
    (being circles, this could be a difference of radii)

    :A: First object
    :B: Second Object
    :returns: Boolean telling if the items collide

    """
    ...


items: list[Item] = [item1, item2, item3, ...]
colliding_items = []

for A in items:
    for B in items:
        if A != B:
            # We avoid checking if an item collides with itself,
            # for obvious reasons
            if is_collision(A, B):
                colliding_items.append((A, B))
