-- Let's think we have a generic set of utilities to work on "vectors"
local vectors = require("vectors")
-- ...
Player = {}

function Player:new(o)
    o = o or {
        MAX_SPEED = 50.0,  -- Maximum speed
        ACCEL = 15.0,  -- The acceleration rate
        DECEL = 30.0,  -- The deceleration rate
        input_accel = {x=0, y=0},  -- Defines the direction we are accelerating
        velocity = {x=0, y=0},  -- Defines the direction and magnitude of our speed
        position = {x=0, y=0},  -- Defines our current position, in (x,y) coordinates
        is_moving = false,  -- Tells us if we're moving
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Player:handle_input()
    -- First of all, we need to zero the input_accel, or we'll be working on "residual data"
    self.input_accel = {x=0, y=0}
    -- Now we can handle movement
    if (KEYBOARD.Left_Arrow_Pressed) then
        self.input_accel.x = self.input_accel.x - 1
    end
    if (KEYBOARD.Right_Arrow_Pressed) then
        self.input_accel.x = self.input_accel.x + 1
    end
    if (KEYBOARD.Down_Arrow_Pressed) then
        self.input_accel.y = self.input_accel.y + 1
    end
    if (KEYBOARD.Right_Arrow_Pressed) then
        self.input_accel.y = self.input_accel.y - 1
    end
    -- If any component of the acceleration vector is not zero, we are moving
    if (self.input_accel.x ~= 0 or self.input_accel.y ~= 0) then
        self.is_moving = true
    end
end

function Player:handle_movement(dt)
    if (self.is_moving) then
        -- Vectors will take care of summing forces
        self.velocity = self.velocity + self.ACCEL * dt * self.input_accel
        -- We need to clamp the speed, to avoid going too fast
        vectors.clamp(self.velocity, self.MAX_SPEED)
    else
        -- We are stopping, let's subtract the deceleration
        local velocity_value = vectors.length(self.velocity) - self.DECEL * dt
        if (velocity_value < 0) then
            -- If, After decelerating, we have a negative value, we need to make it zero or the object will start moving backwards
            velocity_value = 0
        end
        -- We are just changing the length of the vector, so we can just clamp its length
        vectors.clamp(self.velocity, velocity_value)
    end

    -- Now it's time to move the object
    self.position.x = self.position.x + self.velocity.x
    self.position.y = self.position.y + self.velocity.y
end
