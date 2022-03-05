local function LazySingleton(superclass)
    local o = {}
    o.__index = o
    setmetatable(o, superclass)

    function o.new()
        -- We overwrite the constructor to something that returns
        -- the instance if already existing
        if o._instance then
            return o._instance
        end

        local instance = setmetatable({}, o)

        -- Multi-threading: manage race conditions
        -- ----- Critical region start -----
        o._instance = instance
        return o._instance
        -- ----- Critical region end -----
    end

    return o
end
