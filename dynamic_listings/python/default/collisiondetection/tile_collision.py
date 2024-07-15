from math import floor
TILE_WIDTH = 32
TILE_HEIGHT = 32


class Rectangle:

    """
    Represents a rectangle with an upper left corner, width and height
    """

    def __init__(self, corner, width, height):
        """
        Initializes the rectangle

        :corner: The rectangle upper left corner
        :width: The rectangle width
        :height: The rectangle height

        """
        self._corner = corner
        self._width = width
        self._height = height


def convert_pixels_to_tile(x, y):
    """
    Converts a point into tile coordinates

    :x: The pixel X coordinate
    :y: The pixel Y coordinate
    :returns: A 2-tuple containing the tile coordinates detected

    """
    tile_x = floor(x / TILE_WIDTH)
    tile_y = floor(y / TILE_HEIGHT)
    return (tile_x, tile_y)


# We assume the player is falling, so no check will be shown here
points_to_check = [
    Point(player.corner.x, player.corner.y + player.height),
    Point(player.corner.x + player.width, player.corner.y + player.height),
]
for point in points_to_check:
    detected_tile_coordinates = convert_pixels_to_tile(point.x, point.y)
    detected_tile = level.get_tile(detected_tile_coordinates)
    if AABB(player, detected_tile.rectangle):
        # React to the collision
        pass
