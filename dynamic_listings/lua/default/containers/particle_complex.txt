-- This is a simple particle class, it contains a reference to
-- its texture, as well as some state
Particle = {}

function Particle:new(o)
    o = o or {
        -- The particle texture
        texture = nil,
        -- Position, velocity and acceleration on the 2D plane
        position = {x = 0, y = 0},
        velocity = {x = 0, y = 0},
        acceleration = {x = 0, y = 0},
        -- Lifespan of the particle
        lifespan = 0.0,
        -- Current angle of rotation, and relative velocity and acceleration
        rotation = 0.0,
        angular_velocity = 0.0,
        angular_acceleration = 0.0,
        -- Initial Status, for resetting
        initial_velocity = {x=0, y=0},
        initial_rotation = 0.0,
        initial_a_vel = 0.0,
        initial_lifespan = 0.0,
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
    -- Then the rotation
    self.angular_velocity = self.angular_velocity + self.angular_acceleration ;
    self.rotation = (self.rotation  + self.angular_velocity * dt) % 360;  -- Wrap to zero when at 360 degrees
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

function Particle:reset()
    -- This function resets the initial status of the particle
    self.velocity = self.initial_velocity;
    self.rotation = self.initial_rotation;
    self.a_vel = self.initial_a_vel;
    self.lifespan = self.initial_lifespan;
end
