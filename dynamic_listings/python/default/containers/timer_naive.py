class Timer:
    """
    ...
    This is the same as the previous version
    ...
    """

    def update(self, dt: float):
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
            # We reset the timer differently, by adding the "set time"
            self.time += self.set_time
