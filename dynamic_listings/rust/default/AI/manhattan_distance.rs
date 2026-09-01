struct Tile {
    x: i32,
    y: i32,
}

fn manhattan_distance(start: &Tile, goal: &Tile) -> i32 {
    (start.x - goal.x).abs() + (start.y - goal.y).abs()
}
