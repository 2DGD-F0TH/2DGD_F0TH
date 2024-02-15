function is_collision(A, B){
    // Defines how two items collide (being circles, this could be a difference of radii)
    // ...
}

let items = [item1, item2, ...];

function get_colliding_items(items_to_check){
    colliding_items = [];

    for (const A in items_to_check){
        for (const B in items_to_check){
            if (A != B){
                // We avoid checking if an item collides with itself, for obvious reasons
                if (is_collision(A, B)){
                    // We keep together the pair of items that collided
                    colliding_items.push([A, B]);
                }
            }
        }
    }
    return colliding_items;
}
