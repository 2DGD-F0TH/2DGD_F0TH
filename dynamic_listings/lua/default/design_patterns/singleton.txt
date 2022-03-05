local function Singleton(superclass)
    local o = {}
    o.__index = o
    setmetatable(o, superclass)
    local instance = setmetatable({}, o)

    o._instance = instance

    function o.new()
        -- We overwrite the constructor to something that returns
        -- the instance if already existing
        return o._instance
    end

    return o
end
