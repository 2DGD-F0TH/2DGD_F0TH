-- This is a simple timer class that executes a function after
-- a certain amount of time
Timer = {}

function Timer:new(o)
    o = o or {
        time = 0,
        set_time = 0,
        function_to_execute = nil,
        one_shot = false,
        active = false
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

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
        -- We reset the timer (we may need to re-activate it manually later)
        self.time = self.set_time
    end
end
