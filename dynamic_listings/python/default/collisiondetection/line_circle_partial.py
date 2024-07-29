class Point:
    """
    A simple Point
    """

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


def line_circle_collision(circle: Circle, line: Line) -> bool:
    """
    Defines if a line collides with a circle

    :circle: The Circle To check collision for
    :line: The line which we should check collision for
    :returns: A boolean telling us if the line and circle collide
    """
    collides_with_A: bool = circle_point_collision(circle, line.A)
    collides_with_B: bool = circle_point_collision(circle, line.B)
    if collides_with_A or collides_with_B:
        return True
    # ...
