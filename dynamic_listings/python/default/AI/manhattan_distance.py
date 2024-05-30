class Tile:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y


def manhattan_distance(start: Vector2D, goal: Vector2D) -> int:
    return abs(start.x - goal.x) + abs(start.y - goal.y)
