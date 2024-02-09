MyMemoizedObject = {}

function MyMemoizedObject:new()
    o = {
        memory = {}
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function MyMemoizedObject:memoizedfunction(parameter)
    if self.memory[parameter] ~= nil then
        -- If the result was calculated earlier, we can just return it
        return self.memory[parameter];
    end
    -- If the result has never been calculated we do so.
    -- ...
    -- Very complex and heavy calculations here
    -- ...
    local result = something_complex;
    -- Now we save the result in our memory, so other calls with the same parameter will be faster
    self.memory[parameter] = result;
    return result;
end
