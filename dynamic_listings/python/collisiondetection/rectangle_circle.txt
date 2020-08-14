from math import sqrt


class Point(object):

    """A simple Point"""

    def __init__(self, x, y):
        """
        Creates the point
        :x: The X Coordinate of the point
        :y: The Y Coordinate of the point
        """
        self.x = x
        self.y = y


class Rectangle(object):

    """A rectangle, made with an upper-left corner, width and height"""

    def __init__(self, point, width, height):
        """Creates the rectange

        :point: The upper left corner point of the rectangle
        :width: The rectangle Width
        :height: The rectangle Height

        """
        self._point = point
        self._width = width
        self._height = height


class Circle(object):

    """Defines a circle, with center and radius"""

    def __init__(self, center, radius):
        """
        Initializes the circle

        :center: Center of the circle, as a point
        :radius: Radius of the circle, as a float

        """
        self._center = center
        self._radius = radius


def circle_rectangle_collision(circ, rect):
    """
    Detects a collision between a circle and a rectangle

    :circ: The circle to test for collision
    :rect: The rectangle to test against
    :returns: A boolean that tells us whether a collision has happened

    """
    # These variables are used as the coordinates we should test against
    # They are temporarily set to the circle center's coordinates for
    # a reason we'll see soon
    tx = circ.center.x
    ty = circ.center.y

    # Let's detect which edge to test against on the x axis
    if (circ.center.x < rect.corner.x):
        # We're at the left of the rectangle, test against the left side
        tx = rect.corner.x
    elif (circ.center.x > rect.corner.x + rect.width):
        # We're at the right of the rectangle, test against the right side
        tx = rect.corner.y + rect.width

    # Same thing on the vertical axis
    if (circ.center.y < rect.corner.y):
        # We're above the rectangle, test against the top side
        ty = rect.corner.y
    elif (circ.center.y > rect.corner.y + rect.height):
        # We're below the rectangle, test against the bottom side
        ty = rect.corner.y + rect.height

    # Let's get the distance between the testing coordinates
    # and the circle center
    distanceX = circ.center.x - tx
    distanceY = circ.center.y - ty
    distance = sqrt(distanceX**2 + distanceY**2)

    # Note that if the center of the circle is inside the rectangle,
    # the testing coordinates will be the circle's center itself,
    # thus the next conditional will always return true

    if (distance <= circ.radius):
        return True

    # Default to false in case no collision occurs
    return False
