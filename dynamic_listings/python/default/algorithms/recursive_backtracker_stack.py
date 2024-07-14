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
        self.carved_passages: list[tuple[int]] = []

    # ...
    # Same as the previous example
    # ...

    def draw_maze(self, x: int, y: int) -> None:
        """
        Draws a maze using the recursive backtracker algorithm and
        system stack recursion.

        This version will dig walls 2 cells at a time, thus leaving some cells
        acting as "walls" instead of having a more complex structure.
        """
        # First thing, we push our cell into our "carved passages",
        # this will simulate the system stack
        self.carved_passages.append((x, y))
        # When the "carved_passages" array is empty, we are back at the
        # start of the maze
        stored_x: int = x
        stored_y: int = y
        next_x: int = None
        next_y: int = None
        while self.carved_passages:
            # We dig a path in the current cell
            self.make_path(stored_x, stored_y)
            # We create a list of available directions (x and y)
            directions: list[tuple[int]] = [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1)
            ]
            # And we shuffle them
            random.shuffle(directions)

            # While there is at least one direction available
            while directions:
                # We take the last item in directions list (which is random)
                direction_to_try = directions.pop()

                # Calculate the new node coordinates using the chosen direction.
                # We are doubling the movement in each direction, so some cells
                # can act as walls
                next_x = stored_x + (direction_to_try.x * 2)
                next_y = stored_y + (direction_to_try.y * 2)

                # If the node we found is a wall, it means we didn't visit it
                # (and it's inside our maze boundaries)
                if self.is_wall(next_x, next_y):
                    # We have found a new node to dig towards

                    # Since we are moving 2 cells at a time, we need to carve
                    # the cell that "links" our "current node" and the "next node"
                    link_cell_x = stored_x + direction_to_try.x
                    link_cell_y = stored_y + direction_to_try.y
                    self.make_path(link_cell_x, link_cell_y)

                    # Repeat the carving process with the new coordinates
                    # we do this by pushing into the stack
                    self.carved_passages.append((next_x, next_y))
            # If all available directions are exhausted, we pop
            # one layer from our stack: we go back one step
            if not directions:
                popped_vector = self.carved_passages.pop()
                stored_x = popped_vector.x
                stored_y = popped_vector.y
            else:
                # If not, we continue
                stored_x = next_x
                stored_y = next_y
