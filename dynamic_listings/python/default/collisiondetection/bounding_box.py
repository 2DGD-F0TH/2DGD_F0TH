from typing import Self


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


class Rectangle:
    """
    A rectangle, made with an upper-left corner, width and height
    """

    def __init__(self, point: Point, width: int, height: int) -> None:
        """Creates the rectange

        :point: The upper left corner point of the rectangle
        :width: The rectangle Width
        :height: The rectangle Height

        """
        self.corner: Point = point
        self.width: int = width
        self.height: int = height

    @staticmethod
    def from_points(topleft: Point, bottomright: Point) -> Self:
        ...


def bounding_box(vertices: list[Point]) -> Rectangle:
    # First we create and bootstrap the variables
    xmin: int = vertices[0].x
    xmax: int = vertices[0].x
    ymin: int = vertices[0].y
    ymax: int = vertices[0].y
    # Now we iterate through all the other vertices
    for vertex in vertices:
        xmin = min(xmin, vertex.x)
        xmax = max(xmax, vertex.x)
        ymin = min(ymin, vertex.y)
        ymax = max(ymax, vertex.y)
    # Now we can build the needed points for the bounding box
    A: Point = Point(xmin, ymin)
    C: Point = Point(xmax, ymax)
    # We build our bounding box
    boundingBox: Rectangle = Rectangle.from_points(A, C)
    # and return it
    return boundingBox
