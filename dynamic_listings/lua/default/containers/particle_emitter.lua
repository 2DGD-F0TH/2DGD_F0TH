-- This is a simple particle emitter, it contains a list
-- of particles and it updates and manages them
Emitter = {}

function Emitter:new(o)
    o = o or {
        origin = {x=0, y=0},
        particles = {}
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Emitter:update(dt)
    -- Update the entire system, by updating each particle
    for i = 1, #self.particles do
        local particle = self.particles[i]
        if (not particle.is_dead()) then
            particle.update(dt);
        end
    end
end
