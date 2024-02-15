TILE_WIDTH = 32
TILE_HEIGHT = 32

local function convert_pixels_to_tile(x, y)
    -- Converts a point into tile coordinates
    local tile_x = math.floor(x / TILE_WIDTH)
    local tile_y = math.floor(y / TILE_HEIGHT)
    return {tile_x, tile_y}
end
