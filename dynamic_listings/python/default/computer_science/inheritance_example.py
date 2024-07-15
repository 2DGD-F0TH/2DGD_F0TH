from math import pi


class Shape:
    # An abstract shape class
    def area() -> float:
        # An abstract function that will be overridden by subclasses
        raise NotImplementedError

    def perimeter() -> float:
        # An abstract function that will be overridden by subclasses
        raise NotImplementedError


class Rectangle(Shape):
    # A simple rectangle class
    width: float = 0.0
    height: float = 0.0

    def __init__(self, width: float, height: float) -> None:
        self.width = width
        self.height = height

    def area(self) -> float:
        # Returns the Area of the rectangle
        return self.width * self.height

    def perimeter(self) -> float:
        # Returns the Perimeter of the rectangle
        return 2 * (self.width + self.height)


class Circle(Shape):
    # A simple circle class
    radius: float = 0.0

    def __init__(self, radius: float) -> None:
        self.radius = radius

    def area(self) -> float:
        # Returns the Area of the circle
        return pi * self.radius ** 2

    def perimeter(self) -> float:
        # Returns the circumference of the circle
        return 2 * pi * self.radius
