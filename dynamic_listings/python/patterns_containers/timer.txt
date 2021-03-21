class Timer:
    """
    This is a simple timer class that executes a function after
    a certain amount of time
    """
    time = 0
    set_time = 0
    function_to_execute = None
    one_shot = False
    active = False

    def __init__(self, time, function, one_shot=False, active=False):
        # We prepare the timer and memorize the setting
        self.time = time
        self.set_time = time
        # The function pointer should already be prepared with the arguments
        self.function_to_execute = function
        # Is this timer one-shot then disable?
        self.one_shot = one_shot
        # Does this timer need to be active when constructed?
        self.active = active

    def update(self, dt):
        """
        A simple update function for out timer
        """
        if not self.active:
            # We return directly if the timer is disabled
            return
        # Like any other entity, we update it
        self.time -= dt
        # When the timer "ticks", we execute the function
        if self.time <= 0:
            self.function_to_execute()
            if self.one_shot:
                # If this time is a one-shot, we disable it
                self.active = False
            # We reset the timer (we may need to re-activate it manually later)
            self.time = self.set_time
