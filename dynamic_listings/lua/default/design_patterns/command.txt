-- This is the abstract class that will be used as interface
Command = {}

function Command:new(o)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end

function Command:execute()
    -- This method is empty
end

JumpCommand = Command:new()

function JumpCommand:execute()
    -- This will implement the execute method
    self.jump()
end

function JumpCommand:jump()
    -- DO STUFF
end
