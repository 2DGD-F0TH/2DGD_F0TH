from math import sqrt


class Point:

    """A simple Point"""

    def __init__(self, x: int, y: int) -> None:
        """
        Creates the point
        :x: The X Coordinate of the point
        :y: The Y Coordinate of the point
        """
        self.x: int = x
        self.y: int = y


class Line:
    """
    Defines a line, drawn between two points
    """

    def __init__(self, A: Point, B: Point) -> None:
        """
        Initializes a line between 2 points

        :A: First point of the line
        :B: Second Point of the Line

        """
        self.A: Point = A
        self.B: Point = B


class Circle:
    """
    Defines a circle, with center and radius
    """

    def __init__(self, center: Point, radius: float) -> None:
        """
        Initializes the circle

        :center: Center of the circle, as a point
        :radius: Radius of the circle, as a float

        """
        self.center: Point = center
        self.radius: float = radius


def circle_point_collision(circle: Circle, point: Point) -> bool:
    """
    Defines if a circle and a point are colliding
    """
    # Already defined earlier...
    ...


def line_point_collision(line: Line, point: Point) -> bool:
    """
    Defines if a line and a point are colliding
    """
    # Already defined earlier...
    ...


def distance(A: Point, B: Point) -> float:
    """
    Calculates the distance between two points

    :A: First point
    :B: Second Point
    :returns: A float that represents the distance between A and B

    """
    return sqrt((A.x - B.x)**2 + (A.y - B.y)**2)


def line_circle_collision(line: Line, circle: Circle) -> bool:
    """
    Defines if a line collides with a circle

    :circle: The Circle To check collision for
    :line: The line which we should check collision for
    :returns: A boolean telling us if the line and circle collide
    """
    collides_A: bool = circle_point_collision(circle, line.A)
    collides_B: bool = circle_point_collision(circle, line.B)
    if collides_A or collides_B:
        return True
    # ...
    # We pre-calculate "u", we'll use some variables for readability
    x1: float = line.A.x
    x2: float = line.B.x
    xk: int = circle.center.x
    y1: float = line.A.y
    y2: float = line.B.y
    yk: int = circle.center.y
    u: float = (
        ((xk - x1) * (x2 - x1) + (yk - y1) * (y2 - y1))
        / (distance(line.A, line.B))**2
    )
    # Now let's calculate the x and y coordinates
    x: float = x1 + u * (x2 - x1)
    y: float = y1 + u * (y2 - y1)
    # "Reuse", we'll use some older functions, let's create a point,
    # with the coordinates we found
    P: Point = Point(x, y)
    # Let's check if the "closest point" we found is on the line
    if not line_point_collision(line, P):
        # If the point is outside the line, we return false,
        # because the ends have already been checked against collisions
        return False

    # Let's Reuse the Point/Circle Algorithm
    return circle_point_collision(circle, P)
