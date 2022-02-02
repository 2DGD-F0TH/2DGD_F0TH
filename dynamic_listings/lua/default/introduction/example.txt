function Example(phrase)
    print(phrase)
end

-- This is a simple example class
ExampleClass = {}

function ExampleClass:new(o)
    -- This is an example constructor
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end
