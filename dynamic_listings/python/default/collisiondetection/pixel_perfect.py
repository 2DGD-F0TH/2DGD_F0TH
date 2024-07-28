class Color:

    """
    Defines a color
    """

    def __init__(self, colorData: int = 0x0) -> None:
        """Initializes the color """
        self.colorData: int = colorData

    def is_white(self) -> bool:
        return self.colorData == 0xFFFFFF


class Bitmask:

    """A sprite bitmask"""

    def __init__(self, data: list[Color] = None) -> None:
        """Initializes the bitmask with an array of colors

        :data: An array of colors

        """
        self._data: list[Color] = data

    def get_color(self, x: int, y: int) -> Color:
        """
        Gets the color at a certain x and y of the bitmap

        :x: x coordinate of the pixel to return
        :y: y coordinate of the pixel to return
        :returns: The color found

        """
        # Just an example
        ...

class Sprite:
    def __init__(self, bitmask: Bitmask, x: int, y: int, width: int, height: int) -> None:
        self.bitmask: Bitmask = bitmask
        self.x: int = x
        self.y: int = y
        self.width: int = width
        self.height: int = height

def pixel_perfect_collision(A: Sprite, B: Sprite) -> bool:
    """
    Calculates a pixel-perfect collsion

    :A: First sprite
    :B: Second Sprite
    :returns: A boolean telling us if the two sprites collide

    """
    # Calculate the intersecting rectangle to limit checks
    x1: int = max(A.x, B.x)
    x2: int = min((A.x + A.width), (B.x + B.width))

    y1: int = max(A.y, B.y)
    y2: int = min((A.y + A.height), (B.y + B.height))

    # For each pixel in the intersecting rectangle, let's check
    for y in range(y1, y2):
        for x in range(x1, x2):
            a: Color = A.bitmask.get_color(x - A.x, y - A.y)
            b: Color = B.bitmask.get_color(x - B.x, y - B.y)

            if (a.isWhite() and b.isWhite()):
                return True

    # If no collision has occurred by the end of the checking, we're safe
    return False
