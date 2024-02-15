-- This is a simple particle class, now it has some mass
-- and a force application function
Particle = {}

function Particle:new(o)
    o = o or {
        -- ...
        acceleration = {x=0, y=0},
        -- ...
        mass = 0.0,
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Particle:applyForce(force)
    -- This function influences the acceleration by applying force
    local da = force / self.mass
    self.acceleration = self.acceleration + da
end
