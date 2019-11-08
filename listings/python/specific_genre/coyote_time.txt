class Player:
    coyote_time_started = False
    coyote_time = 0
    has_jumped = False
    on_ground = False
    # ...

    def update(self, dt):
        # ...
        if self.onground:
            # Do stuff when player is on ground
            # ...
            pass
        else:
            if not self.has_jumped:
                # Player is not on ground and has not jumped, the player's falling
                if not self.coyote_time_started:
                    self.coyote_time_started = True
                    self.coyote_time = 5
                else:
                    self.coyote_time -= dt

    def jump(self):
        # This function takes care of jumping
        # ...
        if self.coyote_time > 0:
            # Do Jump
            pass
        # ...
