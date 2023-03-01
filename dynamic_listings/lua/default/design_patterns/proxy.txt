-- This class represents a user, with their permissions
User = {}

function User:can_do_thing()
    -- Changes according to user permissions
    -- ...
end

-- This class represents a web request
Request = {}

function Request:new(user)
    o = {
        user = user
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

-- Represents a call to a web page
WebPage = {}

function WebPage:new(o)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end

function WebPage:get(request)
    -- This function gets implemented in the concrete classes;
end

-- Represents an authentication proxy for a web page
WebPageProxy = WebPage:new()

function WebPageProxy:get(request)
    -- Get the requesting user
    local requesting_user = request.user
    if requesting_user.can_do_thing(request) then
        return WebPage:get(request)
    else
        return HttpResponse:new("This user cannot access this page")
    end
end

function main()
    local user = User:new();
    -- The following line is valid due to polimorphism
    local page = WebPageProxy:new();
    -- If the user has permission the page will come through.
    page.get(Request:new(user));
end
