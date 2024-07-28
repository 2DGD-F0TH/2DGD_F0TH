from math import sqrt


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


def distance(A: Point, B: Point) -> float:
    """
    Calculates the distance between two points

    :A: First point
    :B: Second Point
    :returns: A float that represents the distance between A and B

    """
    return sqrt((A.x - B.x)**2 + (A.y - B.y)**2)


def line_point_collision(line: Line, point: Point) -> bool:
    """
    Calculates a possible line/point collision

    :line: A line
    :point: A point
    :returns: A boolean telling us if the point and line collide

    """
    # First, let's calculate the length of the line
    length: float = distance(line.A, line.B)
    # Now let's calculate the distance between the point pt
    # and the point "A" of the line
    point_to_a: float = distance(line.A, point)
    # Same Goes for the distance between pt and "B"
    point_to_b: float = distance(line.B, point)
    # Now for the detection
    
    is_colliding = point_to_a + point_to_b == length
    return is_colliding
