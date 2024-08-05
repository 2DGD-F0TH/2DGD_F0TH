from math import sqrt


class Tile:
    def __init__(self, x: int, y: int) -> None:
        self.x: int = x
        self.y: int = y


def euclidean_distance(start: Vector2, goal: Vector2) -> float:
    return sqrt((start.x - goal.x) ** 2 + (start.y - goal.y) ** 2)
