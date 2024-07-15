class Point:

    """A simple Point"""

    def __init__(self, x: int, y: int):
        """
        Creates the point
        :x: The X Coordinate of the point
        :y: The Y Coordinate of the point
        """
        self.x = x
        self.y = y


class Line:

    """Defines a line, drawn between two points"""

    def __init__(self, A: Point, B: Point):
        """
        Initializes a line between 2 points

        :A: First point of the line
        :B: Second Point of the Line

        """
        self.A = A
        self.B = B


class Circle:

    """Defines a circle, with center and radius"""

    def __init__(self, center: Point, radius: int):
        """
        Initializes the circle

        :center: Center of the circle, as a point
        :radius: Radius of the circle, as a float

        """
        self._center = center
        self._radius = radius


def circle_point_collision(circle: Circle, point: Point):
    """
    Defines if a circle and a point are colliding
    """
    # Already defined earlier...
    ...


def line_circle_collision(circle: Circle, line: Line) -> bool:
    """
    Defines if a line collides with a circle

    :circle: The Circle To check collision for
    :line: The line which we should check collision for
    :returns: A boolean telling us if the line and circle collide

    """
    collides_with_A = circle_point_collision(circle, line.A)
    collides_with_B = circle_point_collision(circle, line.B)
    if collides_with_A or collides_with_B:
        return True
    # ...
