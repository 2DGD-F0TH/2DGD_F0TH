from math import sqrt


class Tile:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y


def euclidean_distance(start: Vector2D, goal: Vector2D) -> float:
    return sqrt((start.x - goal.x) ** 2 + (start.y - goal.y) ** 2)
