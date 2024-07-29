class Tile:
    # Our "Tile" Class
    pass


class TwoDimensionGrid:
    # Represents a 2D grid of "Tile" classes
    grid: list[list[Tile]] = None
    width: int = 0
    height: int = 0

    def __init__(self, rows: int, cols: int) -> None:
        # Prepares the memory for the grid
        self.grid: list[list[Tile]] = [
            [Tile() for _ in range(cols)] for _ in range(rows)
        ]
        self.height: int = rows
        self.width: int = cols

    def get_cell(self, row: int, col: int) -> Tile | None:
        # Gets a cell from the 2D Grid
        if all(row >= 0, row < self.height, col >= 0, col < self.width):
            # We better check if we are inside the grid
            return self.grid[row][col]
        return None

    def get_adjacent_cells(self, row: int, col: int) -> list[Tile]:
        """
        Returns a list of cells adjacent the ones we give
        REMEMBER: We index at 0 so the first row is 0, the last one is at
        "height - 1", same goes for columns
        """
        result: list = []
        if all(row >= 0, row < self.height, col >= 0, col < self.width):
            # We better check if we are inside the grid
            if row > 0:
                # We are not on the first row, we can add the cell above
                result.append(self.get_cell(row - 1, col))
            if row < self.height - 1:
                # We are not on the last row, we can add the cell below
                result.append(self.get_cell(row + 1, col))
            if col > 0:
                # We are not on the first column, we can add the cell on the left
                result.append(self.get_cell(row, col - 1))
            if col < self.width - 1:
                # We are not on the last column, we can add the cell on the right
                result.append(self.get_cell(row, col + 1))
        # If the checks went well, toReturn will have
        # a list of the adjacent cells, if not it will be empty
        return result
