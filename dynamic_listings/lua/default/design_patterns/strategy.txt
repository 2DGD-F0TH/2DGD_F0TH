-- This class defines the strategy interface the client will refer to
Strategy = {}

function Strategy:new(o)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end

function Strategy:algorithm()
    -- This algorithm will be implemented by the subclasses
end

ConcreteStrategy1 = Strategy:new()

function ConcreteStrategy1:algorithm()
    -- Real implementation of the algorithm
    -- DO STUFF
end

ConcreteStrategy2 = Strategy:new()

function ConcreteStrategy2:algorithm()
    -- Real implementation of the algorithm
    -- DO STUFF SLIGHTLY DIFFERENTLY
end

-- Example Usage
local function main()
    local to_execute = nil
    if (condition) then
        to_execute = ConcreteStrategy1:new()
    else
        to_execute = ConcreteStrategy2:new()
    end
    to_execute.algorithm()  -- This will execute 1 or 2 depending on "condition"
end
