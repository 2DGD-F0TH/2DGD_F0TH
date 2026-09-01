struct Tile {
    x: u32,
    y: u32,
}

fn euclidean_distance(start: &Tile, goal: &Tile) -> f32 {
    ((start.x - goal.x).pow(2) as f32 + (start.y - goal.y).pow(2) as f32).sqrt()
}
