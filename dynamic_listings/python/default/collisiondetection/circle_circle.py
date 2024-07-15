from math import sqrt


class Circle:

    """Defines a circle, with center and radius"""

    def __init__(self, center: Point, radius: float) -> None:
        """
        Initializes the circle

        :center: Center of the circle, as a point
        :radius: Radius of the circle, as a float

        """
        self.center = center
        self.radius = radius


def get_distance(A: Point, B: Point) -> float:
    """
    Calculates the distance between two points

    :A: First point
    :B: Second Point
    :returns: A float that represents the distance between A and B

    """
    return sqrt((A.x - B.x)**2 + (A.y - B.y)**2)


def circle_circle_collision(A: Circle, B: Circle) -> bool:
    """
    Calculates the collision between two circles

    :A: First Circle
    :B: Second Circle
    :returns: A boolean telling if the circles collide

    """
    is_colliding = get_distance(A.center, B.center) <= A.radius + B.radius
    return is_colliding
