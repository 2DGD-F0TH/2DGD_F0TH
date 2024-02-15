class Enemy:
    """
    Represents a simple enemy
    """
    # ...
    PURSUETIME = 10.0
    position_x = 0.0
    position_y = 0.0
    pursue_timer = Timer()
    brain = FSM()

    def __init__(self, x, y):
        """
        Constructor
        """
        self.position_x = x
        self.position_y = y
        self.brain.setState(self.patrol)

    def sees(self, other):
        """
        Implements logic for the "sight" of the enemy
        """
        # ...
        pass

    def patrol(self, dt):
        # Normal patrolling of the enemy
        # Move, turn, path find...
        if self.sees(player):
            # ...
            # Pursue for xx seconds
            self.pursue_timer.set(self.PURSUETIME)
            self.pursue_timer.start()
            # Change FSM State
            self.brain.setState(self.pursue)

    def pursue(self, dt):
        # Tries to pursue the enemy
        if self.sees(player):
            # Continue Pursuing, by resetting the timer
            self.pursue_timer.set(self.PURSUETIME)
            # ...
        # ...
        # If the enemy is not in sight for xx seconds
        if (self.pursue_timer.is_finished()):
            # go back to patrolling
            self.brain.setState(self.patrol)

    def update(self, dt):
        # The enemy update function
        # ...
        self.pursue_timer.update(dt)
        self.brain.update(dt)
        # ...
