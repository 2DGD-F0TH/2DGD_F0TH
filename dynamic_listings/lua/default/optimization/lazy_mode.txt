LazyObject = {}

function LazyObject:new(nums)
    local o = {
        numbers = nums
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function LazyObject:getObject(index)
    -- Calculates the halved number on-demand
    return self.numbers[index / 2];
end
