Timer = {}
-- ...
-- This is the same as the older version
-- ...
function Timer:update(dt)
    if (not self.active) then
        -- We return directly if the timer is disabled
        return
    end
    -- Like any other entity, we update it
    self.time = self.time - dt
    -- When the timer "ticks", we execute the function
    if (self.time <= 0) then
        self.function_to_execute()
        if (self.one_shot) then
            -- If this timer is a one-shot, we disable it
            self.active = false
        end
        -- We reset the timer differently, by adding the "set time" until we have a positive value
        while (self.time <= 0) do
            self.time = self.time + self.set_time
        end
    end
end
