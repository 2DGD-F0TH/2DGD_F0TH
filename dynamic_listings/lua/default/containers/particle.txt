-- This is a simple particle class, it contains a reference to
-- its texture, as well as some state
Particle = {}

function Particle:new(o)
    o = o or {
        texture = nil,
        position = {x = 0, y = 0},
        velocity = {x = 0, y = 0},
        acceleration = {x = 0, y = 0},
        lifespan = 0.0
    }
    setmetatable(o, self)
    self.__index = self
    return o
end


function Particle:update(dt)
    -- We update the velocity (assuming dt is in milliseconds)
    self.velocity = self.velocity + self.acceleration
    -- Then the position
    self.position = self.position + self.velocity * dt
    -- Now we update the lifespan of the particle
    self.lifespan = self.lifespan - dt
end

function Particle:is_dead()
    -- Returns a boolean representing if the particle is dead
    return self.lifespan <= 0
end

function Particle:setPosition(position)
    -- Sets the particle position
    self.position = position
end
