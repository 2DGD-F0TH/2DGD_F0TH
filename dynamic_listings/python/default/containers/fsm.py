from typing import Callable


class FSM:
    """
    This class defines a Finite State Machine
    The currently active state is represented by a function
    pointer
    """

    current_state: Callable = None

    def set_state(self, f: Callable) -> None:
        """
        Sets the state, from this point on, update will
        change its strategy
        """
        self.current_state = f

    def update(self, dt: float) -> None:
        # If there is a current state, execute it
        if self.current_state:
            self.current_state(dt)
