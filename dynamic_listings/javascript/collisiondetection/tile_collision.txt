const TILE_WIDTH = 32;
const TILE_HEIGHT = 32;


class Rectangle{
    // A rectangle will represent the player
    constructor(){
        this.corner = new Point();
        this.width = 0;
        this.height = 0;
    }
}

function convert_pixels_to_tile(x, y){
    // Converts a point into tile coordinates
    let tile_x = Math.floor(x / TILE_WIDTH);
    let tile_y = Math.floor(y / TILE_HEIGHT);
    let to_return = [tile_x, tile_y];
    return to_return;
}

// We assume the player is falling, so no check will be shown here
let points_to_check = [
    Point(player.corner.x, player.corner.y + player.height),
    Point(player.corner.x + player.width, player.corner.y + player.height),
]
for (const point in points_to_check){
    let detected_tile_coordinates = convert_pixels_to_tile(point.x, point.y);
    let detected_tile = get_tile(detected_tile_coordinates[0], detected_tile_coordinates[1]);
    if (AABB(player, detected_tile.rectangle)){
        // React to the collision
        // ...
    }
}
