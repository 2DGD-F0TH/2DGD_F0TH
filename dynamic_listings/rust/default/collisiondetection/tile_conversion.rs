const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;

fn convert_pixels_to_tile(x: u32, y: u32) -> [u32; 2] {
    // Converts a point into tile coordinates
    let tile_x = x / TILE_WIDTH;
    let tile_y = y / TILE_HEIGHT;
    [tile_x, tile_y]
}
