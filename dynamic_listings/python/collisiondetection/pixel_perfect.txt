class Color(object):

    """
    Defines a color
    """

    def __init__(self, colorData=0x0):
        """Initializes the color """
        self.colorData = colorData

    def isWhite(self):
        return self.colorData == 0xFFFFFF


class Bitmask(object):

    """A sprite bitmask"""

    def __init__(self, data=[]):
        """Initializes the bitmask with an array of colors

        :data: An array of colors

        """
        self._data = data

    def getColor(self, x, y):
        """
        Gets the color at a certain x and y of the bitmap

        :x: x coordinate of the pixel to return
        :y: y coordinate of the pixel to return
        :returns: The color found

        """
        # Just an example
        pass


def pixel_perfect_collision(A, B):
    """
    Calculates a pixel-perfect collsion

    :A: First sprite
    :B: Second Sprite
    :returns: A boolean telling us if the two sprites collide

    """
    # Calculate the intersecting rectangle to limit checks
    x1 = max(A.x, B.x)
    x2 = min((A.x + A.width), (B.x + B.width))

    y1 = max(A.y, B.y)
    y2 = min((A.y + A.height), (B.y + B.height))

    # For each pixel in the intersecting rectangle, let's check
    for y in range(y1, y2):
        for x in range(x1, x2):
            a = A.bitmask.getColor(x - A.x, y - A.y)
            b = B.bitmask.getColor(x - B.x, y - B.y)

            if (a.isWhite() and b.isWhite()):
                return True

    # If no collision has occurred by the end of the checking, we're safe
    return False
