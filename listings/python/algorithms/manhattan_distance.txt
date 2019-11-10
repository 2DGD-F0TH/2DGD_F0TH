class Tile:
    def __init__(self, x, y):
        self.x = x
        self.y = y


def manhattan_distance(start, goal):
    return abs(start.x - goal.x) + abs(start.y - goal.y)
