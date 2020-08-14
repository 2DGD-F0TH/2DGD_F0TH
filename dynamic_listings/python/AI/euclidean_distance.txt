from math import sqrt


class Tile:
    def __init__(self, x, y):
        self.x = x
        self.y = y


def euclidean_distance(start, goal):
    return sqrt((start.x - goal.x) ** 2 + (start.y - goal.y) ** 2)
