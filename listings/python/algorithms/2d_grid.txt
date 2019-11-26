class Tile:
    # Our "Tile" Class
    pass


class TwoDimension_Grid:
    # Represents a 2D grid of "Tile" classes
    grid = None
    width = 0
    height = 0

    def __init__(self, rows, cols):
        # Prepares the memory for the grid
        self.grid = [[Tile() for _ in range(cols)] for _ in range(rows)]
        self.height = rows
        self.width = cols

    def getCell(self, row, col):
        # Gets a cell from the 2D Grid
        if (row >= 0 and row < self.height) and (col >= 0 and col < self.width):
            # We better check if we are inside the grid
            return self.grid[row][col]
        else:
            return None

    def getAdjacentCells(self, row, col):
        """
        Returns a list of cells adjacent the ones we give
        REMEMBER: We index at 0 so the first row is 0, the last one is at
        "height - 1", same goes for columns
        """
        toReturn = []
        if (row >= 0 and row < self.height) and (col >= 0 and col < self.width):
            # We better check if we are inside the grid
            if (row > 0):
                # We are not on the first row, we can add the cell above
                toReturn.append(self.getCell(row - 1, col))
            if (row < self.height - 1):
                # We are not on the last row, we can add the cell below
                toReturn.append(self.getCell(row + 1, col))
            if (col > 0):
                # We are not on the first column, we can add the cell on the left
                toReturn.append(self.getCell(row, col - 1))
            if (col < self.width - 1):
                # We are not on the last column, we can add the cell on the right
                toReturn.append(self.getCell(row, col + 1))
        """
        If the checks went well, toReturn will have
        a list of the adjacent cells, if not it will be empty
        """
        return toReturn
