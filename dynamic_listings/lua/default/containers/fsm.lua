-- This class defines a Finite State Machine
-- The currently active state is represented by a function
-- pointer
FSM = {}

function FSM:new(o)
    o = o or {
        current_state = nil
    }
    setmetatable(o, self)
    self.__index = self
    return o
end


function FSM:setState(f)
    -- Sets the state, from this point on, update will
    -- change its strategy
    self.current_state = f
end

function FSM:update(dt)
    -- If there is a current state, execute it
    if self.current_state ~= nil then
        self.current_state(dt)
    end
end
