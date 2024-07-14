import random


class Maze:
    def __init__(self, width: int, height: int) -> None:
        """
        A simple constructor
        """
        self.width: int = width  # Needs to be an odd number
        self.height: int = height  # Needs to be an odd number

        # Create a 2D matrix containing the maze data
        # False = Path, True = Wall
        self.cells: list[list[bool]] = [
            [True] * self.width
        ] * self.height

    def make_path(self, x: int, y: int) -> None:
        """
        Given a cell coordinates, sets the cell as a path
        """
        self.cells[y][x] = False

    def make_wall(self, x: int, y: int) -> None:
        """
        Given a cell coordinates, sets the cell as a wall
        """
        self.cells[y][x] = True

    def is_wall(self, x: int, y: int) -> bool:
        """
        Returns true if the given cell coordinates are inside the maze
        boundaries and if the selected cell is a wall
        """
        # Let's check if the coordinates are within the maze grid
        if (0 <= x < self.width) and (0 <= y < self.height):
            # if they are, then we can check if the cell is a wall
            return self.cells[y][x]
        # If we're outside the maze boundaries, we return false
        return False

    def draw_maze(self, x: int, y: int) -> None:
        """
        Draws a maze using the recursive backtracker algorithm and
        system stack recursion.

        This version will dig walls 2 cells at a time, thus leaving some cells
        acting as "walls" instead of having a more complex structure.
        """
        # We dig a path in the current cell
        self.make_path(x, y)
        # We create a list of available directions (x and y)
        directions: list[list[int]] = [[1, 0], [-1, 0], [0, 1], [0, -1]]
        # And we shuffle them
        random.shuffle(directions)

        # While there is at least one direction available
        while len(directions) > 0:

            # We take the last item in our directions list (which is random)
            direction_to_try: list[int] = directions.pop()

            # Calculate the new node's coordinates using the chosen direction.
            # We are doubling the movement in each direction, so some cells
            # can act as walls
            next_x: int = x + (direction_to_try[0] * 2)
            next_y: int = y + (direction_to_try[1] * 2)

            # If the node we found is a wall, it means we didn't visit it
            # (and it's inside our maze boundaries)
            if self.is_wall(next_x, next_y):
                # We have found a new node to dig towards

                # Since we are moving 2 cells at a time, we need to carve
                # the cell that "links" our "current node" and the "next node"
                link_cell_x: int = x + direction_to_try[0]
                link_cell_y: int = y + direction_to_try[1]
                self.make_path(link_cell_x, link_cell_y)

                # Repeat the carving process with the new coordinates
                self.draw_maze(next_x, next_y)
        # If all available directions are exhausted, we return (thus popping
        # one layer from the system stack: we go back one step)
        # If we are back at the starting cell, the algorithm terminates
