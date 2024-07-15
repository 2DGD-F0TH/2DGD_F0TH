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


class Rectangle:

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

    @staticmethod
    def from_points(topleft: Point, bottomright: Point) -> Rectangle:
        ...

def bounding_box(vertices: list[Point]) -> Rectangle:
    # First we create and bootstrap the variables
    xmin = vertices[0].x
    xmax = vertices[0].x
    ymin = vertices[0].y
    ymax = vertices[0].y
    # Now we iterate through all the other vertices
    for vertex in vertices:
        if vertex.x < xmin:
            xmin = vertex.x
        if vertex.x > xmax:
            xmax = vertex.x
        if vertex.y < ymin:
            ymin = vertex.y
        if vertex.y > ymax:
            ymax = vertex.y
    # Now we can build the needed points for the bounding box
    A = Point(xmin, ymin)
    C = Point(xmax, ymax)
    # We build our bounding box
    boundingBox = Rectangle.from_points(A, C)
    # and return it
    return boundingBox
