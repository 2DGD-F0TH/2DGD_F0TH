from math import floor
TILE_WIDTH = 32
TILE_HEIGHT = 32


def convert_pixels_to_tile(x: int, y: int) -> tuple[int, int]:
    """
    Converts a point into tile coordinates

    :x: The pixel X coordinate
    :y: The pixel Y coordinate
    :returns: A 2-tuple containing the tile coordinates detected

    """
    tile_x = floor(x / TILE_WIDTH)
    tile_y = floor(y / TILE_HEIGHT)
    return (tile_x, tile_y)
