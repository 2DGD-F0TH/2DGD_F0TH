from random import choice as random_choice
DIRECTIONS: list[str] = ["NORTH", "EAST", "SOUTH", "WEST"]


class AIEntity:
    # 0=North, 1=East, ...
    forward_direction_index: int = 0
    current_cell: Cell = Cell(1, 0)
    # ...

    def get_adjacent_cell(self, cell: Cell, direction: str) -> Cell:
        # Returns the adjacent cell in said direction
        if direction == "NORTH":
            cell.y -= 1
        if direction == "SOUTH":
            cell.y += 1
        if direction == "WEST":
            cell.x -= 1
        if direction == "EAST":
            cell.x += 1
        return cell

    def is_valid(self, cell: Cell) -> bool:
        """
        Returns true if the cell is valid, aka
        does not have a wall and does not go backwards
        """
        if cell.is_wall():
            # The cell is a wall
            return False
        if cell == self.get_adjacent_cell(
                self.current_cell,
                DIRECTIONS[(self.forward_direction_index + 2) % 4]):
            # We're going backwards, we don't want that
            return False
        # In all other cases, it's valid
        return True

    def update(self, dt: float) -> None:
        # ...
        # Choose a random direction
        chosen_direction: str = random_choice(DIRECTIONS)
        i: int = 0
        next_cell: Cell = self.get_adjacent_cell(self.current_cell, chosen_direction)
        while (not self.is_valid(next_cell) and i != 4):
            chosen_direction = DIRECTIONS[
                (DIRECTIONS.index(chosen_direction) + 1) % 4
            ]
            next_cell: Cell = self.get_adjacent_cell(self.current_cell,
                                                     chosen_direction)
            i += 1
        if i == 4:
            # We exhausted the possibilities, go backwards
            chosen_direction = DIRECTIONS[
                (self.forward_direction_index + 2) % 4
            ]
            next_cell = self.get_adjacent_cell(self.current_cell,
                                               chosen_direction)
        # Move
        self.move_to(next_cell)
