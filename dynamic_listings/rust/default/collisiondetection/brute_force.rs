fn is_collision(a: &Item, b: &Item) -> bool {
    // Defines how two items collide (being circles, this could be a difference of radii)
}

let items: Vec<Item> = Vec::new();
let mut colliding_items: Vec<(Item, Item)> = Vec::new();

for a in &items {
    for b in &items {
        if a == b {
            continue;
        }
        if is_collision(&a, &b) {
            colliding_items.push((a.clone(), b.clone()));
        }
    }
}
