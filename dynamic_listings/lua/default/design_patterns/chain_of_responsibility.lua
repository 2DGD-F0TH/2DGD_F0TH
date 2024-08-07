-- This is the handler abstract/class interface that the sender connects to
Handler = {}

function Handler:new(o)
    o = o or {
        next = nil  -- The next handler in the chain
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Handler:handle_request()
    if (condition) then
        -- In case I can handle this request
        return self.real_handler()
    end

    if (next ~= nil) then
        return next.handle_request()
    end
end

function Handler:real_handler()
    -- This function gets implemented in the concrete classes
end

function Handler:add_handler(new_handler)
    next = new_handler
    return next  -- Allows for chaining .add_handler().add_handler()...
end
