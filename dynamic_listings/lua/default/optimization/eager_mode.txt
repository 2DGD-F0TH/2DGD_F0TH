EagerObject = {}

function EagerObject:new(nums)
    local o = {
        numbers = nums
    }
    setmetatable(o, self)
    self.__index = self
    o["halved_numbers"] = {}
    -- Prepares the halved numbers list
    for i = 1, #nums do
        local num = nums[i]
        table.insert(o["halved_numbers"], num/2)
    end
    return o
end

function EagerObject:getObject(index)
    -- Returns the pre-calculated object at the requested index
    return self.halved_numbers[index];
end
