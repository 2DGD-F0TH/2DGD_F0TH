-- Represents a simple enemy
Enemy = {}

function Enemy:new(o)
    -- Constructor
    o = o or {
        PURSUETIME = 10.0,
        position_x = 0.0,
        position_y = 0.0,
        pursue_timer = Timer:new(),
        brain = FSM:new()
    }
    o.brain.setState(self.patrol)
    setmetatable(o, self)
    self.__index = self
    return o
end

function Enemy:sees(other)
    -- Implements logic for the "sight" of the enemy
    -- ...
end

function Enemy:patrol(dt)
    -- Normal patrolling of the enemy
    -- Move, turn, path find...
    if self.sees(player) then
        -- ...
        -- Pursue for xx seconds
        self.pursue_timer.set(self.PURSUETIME)
        self.pursue_timer.start()
        -- Change FSM State
        self.brain.setState(self.pursue)
    end
end

function Enemy:pursue(dt)
    -- Tries to pursue the enemy
    if self.sees(player) then
        -- Continue Pursuing, by resetting the timer
        self.pursue_timer.set(self.PURSUETIME)
        -- ...
    end
    -- ...
    -- If the enemy is not in sight for xx seconds
    if (self.pursue_timer.is_finished()) then
        -- go back to patrolling
        self.brain.setState(self.patrol)
    end
end

function Enemy:update(dt)
    -- The enemy update function
    -- ...
    self.pursue_timer.update(dt)
    self.brain.update(dt)
    -- ...
end
