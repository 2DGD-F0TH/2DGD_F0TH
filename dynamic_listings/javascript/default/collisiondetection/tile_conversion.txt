const TILE_WIDTH = 32;
const TILE_HEIGHT = 32;

function convert_pixels_to_tile(x, y){
    // Converts a point into tile coordinates
    let tile_x = Math.floor(x / TILE_WIDTH);
    let tile_y = Math.floor(y / TILE_HEIGHT);
    let to_return = [tile_x, tile_y];
    return to_return;
}
