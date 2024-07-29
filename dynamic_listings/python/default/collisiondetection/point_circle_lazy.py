from math import sqrt


class Circle:

    """Defines a circle, with center and radius"""

    def __init__(self, center: Point, radius: float) -> None:
        """
        Initializes the circle

        :center: Center of the circle, as a point
        :radius: Radius of the circle, as a float

        """
        self.center: Point = center
        self.radius: float = radius


def distance(A: Point, B: Point) -> float:
    """
    Calculates the distance between two points

    :A: First point
    :B: Second Point
    :returns: A float that represents the distance between A and B

    """
    return sqrt((A.x - B.x)**2 + (A.y - B.y)**2)


def circle_point_collision(A: Circle, B: Point) -> bool:
    """
    Detects a collision between a circle and a point

    :A: The circle being checked
    :B: The point being checked
    :returns: A boolean telling is if the point and circle collided

    """
    return distance(A.center, B) <= A.radius
