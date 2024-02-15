List = {}

function List:new(o)
    o = o or {nodeList = {}}
    setmetatable(o, self)
    self.__index = self
    return o
end

function List:getLength()
    local counter = 0
    for i = 1, #self.nodeList do
        counter = counter + 1
    end
    return counter
end
