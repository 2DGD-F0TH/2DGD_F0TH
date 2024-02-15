FirstService = {}
function FirstService:new(o)
-- Implementation of FirstService here...
end

SecondService = {}
function SecondService:new(o)
-- Implementation of SecondService here...
end

--[[
-- This class hides the complexities of using
-- FirstService and SecondService from the user
-- by "wrapping" them in a comfortable startAll
-- function
--]]
Facade = {}

function Facade:new(o)
    o = o or {
        service1 = FirstService.new({}),
        service2 = SecondService.new({})
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Facade:startAll()
    --[[
    -- The facade starts all the services and does
    -- some status checking, this is hidden from the
    -- user.
    -- Returns true if all services started successfully
    -- false otherwise
    --]]
    local firstServiceStarted = self.service1.start()
    if (firstServiceStarted ~= true) then
        return false
    end
    local secondServiceStarted = self.service2.start()
    if (secondServiceStarted ~= true) then
        return false
    end
    -- Here everything started successfully
    return true;
end
