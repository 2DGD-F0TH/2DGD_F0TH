const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;

struct Rectangle {
    corner: Point,
    width: u32,
    height: u32,
}

fn convert_pixels_to_tile(x: u32, y: u32) -> [u32; 2] {
    // Converts a point into tile coordinates
    let tile_x = x / TILE_WIDTH;
    let tile_y = y / TILE_HEIGHT;
    [tile_x, tile_y]
}

// We assume the player is falling, so no check will be shown here
let points_to_check = vec![
    Point { x: player.corner.x, y: player.corner.y + player.height },
    Point { x: player.corner.x + player.width, y: player.corner.y + player.height },
];

for point in points_to_check {
    let detected_tile_coordinates = convert_pixels_to_tile(point.x, point.y);
    let detected_tile = tile(&detected_tile_coordinates);
    if aabb(&player, &detected_tile.rectangle) {
        // React to the collision
    }
}
