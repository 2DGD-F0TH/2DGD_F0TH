from math import sqrt


class Circle:

    """Defines a circle, with center and radius"""

    def __init__(self, center, radius):
        """
        Initializes the circle

        :center: Center of the circle, as a point
        :radius: Radius of the circle, as a float

        """
        self._center = center
        self._radius = radius


def distance(A, B):
    """
    Calculates the distance between two points

    :A: First point
    :B: Second Point
    :returns: A float that represents the distance between A and B

    """
    return sqrt((A.x - B.x)**2 + (A.y - B.y)**2)


def circle_point_collision(A, B):
    """
    Detects a collision between two circles

    :A: First Circle
    :B: Second Circle
    :returns: A boolean telling is if the two circles collided

    """
    return distance(A.center, B) <= A.radius
