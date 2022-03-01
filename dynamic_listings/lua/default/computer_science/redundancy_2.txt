List = {}

function List:new(o)
    o = o or {nodeList = {}, length=0}
    setmetatable(o, self)
    self.__index = self
    return o
end

function List:getLength()
    return self.length
end

function List:addItem()
    -- ... Normal operation ...
    -- ...
    -- We update our length counter
    self.length = self.length + 1
end

function List:removeItem(node)
    -- ... Normal removal operation ...
    -- ...
    -- We update our length counter
    self.length = self.length - 1
end

function List:clear()
    -- ... Normal clear operation ...
    -- ...
    -- We clear the length too
    self.length = 0
end
