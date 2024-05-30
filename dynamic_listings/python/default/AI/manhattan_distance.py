class Tile:
    def __init__(self, x: int, y: int):
        self.x: int = x
        self.y: int = y


def manhattan_distance(start: Vector2D, goal: Vector2D) -> int:
    return abs(start.x - goal.x) + abs(start.y - goal.y)
