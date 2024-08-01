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

    def get_available_directions(self, cell: Cell) -> list[Cell]:
        # Returns a list of available directions
        result: list[Cell] = []
        for direction in DIRECTIONS:
            if self.is_valid(self.get_adjacent_cell(cell, direction)):
                result.append(direction)
        return result

    def update(self, dt: float) -> None:
        # ...
        # Get a list of the available directions
        available_directions: list[Cell] = self.get_available_directions(
            self.current_cell
        )
        chosen_direction: str = "NORTH"  # Just a default
        if len(available_directions) == 0:
            # No directions are available, let's go back
            chosen_direction = DIRECTIONS[
                (self.forward_direction_index + 2) % 4
            ]
        else:
            # Choose a random direction among the available ones
            chosen_direction = random_choice(available_directions)
        # Move
        next_cell: Cell = self.get_adjacent_cell(
            self.current_cell, chosen_direction
        )
        self.move_to(next_cell)
