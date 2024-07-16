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


class Rectangle:

    """A rectangle, made with an upper-left corner, width and height"""

    def __init__(self, point: Point, width: int, height: int) -> None:
        """Creates the rectange

        :point: The upper left corner point of the rectangle
        :width: The rectangle Width
        :height: The rectangle Height

        """
        self.corner: Point = point
        self.width: int = width
        self.height: int = height


def rect_rect_collision(A: Rectangle, B: Rectangle) -> bool:
    """
    Checks for an axis-aligned bounding rectangle collision

    :A: The First Rectangle
    :B: The Second Rectangle
    :returns: A boolean that tells if the rectangles collided
    """
    
    is_colliding = (A.corner.x < B.corner.x + B.width) and\
       (A.corner.x + A.width > B.corner.x) and\
       (A.corner.y < B.corner.y + B.height) and\
       (A.corner.y + A.height > A.corner.y)
    
    return is_colliding
