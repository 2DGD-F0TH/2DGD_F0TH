-- This is a simple particle emitter, it contains a list
-- of particles and it updates and manages them
Emitter = {}

function Emitter:new(o)
    o = o or {
        origin = {x=0, y=0},
        particles = {},
        -- Defines if this emitter streams continuously or only a burst of particles
        one_shot = false
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Emitter:update(dt)
    -- Update the entire system, by updating each particle
    for i = 1, #self.particles do
        local particle = self.particles[i]
        if (self.one_shot) then
            if (particle.is_dead()) then
                goto continue
            else
                particle.update(dt)
            end
        else
            if (particle.is_dead()) then
                particle.reset()  -- Resets the state of the particle
                particle.setPosition(self.origin)
            end
            particle.update(dt);
        end
        ::continue::
    end
end
