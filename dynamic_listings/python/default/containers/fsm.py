class FSM:
    """
    This class defines a Finite State Machine
    The currently active state is represented by a function
    pointer
    """

    current_state = None

    def setState(self, f):
        """
        Sets the state, from this point on, update will
        change its strategy
        """
        self.current_state = f

    def update(self, dt):
        # If there is a current state, execute it
        if self.current_state is not None:
            self.current_state(dt)
