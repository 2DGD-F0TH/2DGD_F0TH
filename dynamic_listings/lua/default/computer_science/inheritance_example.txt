-- This doesn't work well in Lua due to its lack of classes, but it may look something like this
-- An abstract shape class
Shape = {}

function Shape:new(o)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end

function Shape:area()
    -- An abstract function that will be overridden by subclasses
    error("Not implemented")
end

function Shape:perimeter()
    -- An abstract function that will be overridden by subclasses
    error("Not implemented")
end

-- A simple rectangle class
Rectangle = Shape:new()

function Rectangle:new(o)
    o = o or {
        width = 0.0,
        height = 0.0
    }
    setmetatable(o, self)
    self.__index = self
    return o
end


function Rectangle:area()
    -- Returns the Area of the rectangle
    return self.width * self.height;
end

function Rectangle:perimeter()
    -- Returns the Perimeter of the rectangle
    return 2 * (self.width + self.height);
end

-- A simple circle class
Circle = Shape:new()

function Circle:new(o)
    o = o or {
        radius = 0.0
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Circle:area()
    -- Returns the Area of the circle
    return 3.1415 * 3.1415 * self.radius;
end

function Circle:perimeter()
    -- Returns the circumference of the circle
    return 2 * 3.1415 * self.radius;
end
