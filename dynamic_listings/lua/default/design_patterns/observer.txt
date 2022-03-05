-- This is the observed class that contains the list of observers and
-- the notifyObservers method
Subject = {}

function Subject:new(o)
    o = o or {
        observers = {}
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Subject:register_observer(observer)
    table.insert(self.observers, observer)
end

function Subject:notifyObservers()
    for i = 1, #self.observers do
        local observer = self.observers[i]
        observer.update()
    end
end


-- This is the class that contains the update method, used to force
-- an update in the observer
Observer = {}

function Observer:update()
    print("I have been updated!")
end


local subject = Subject:new()
local observer = Observer:new()
subject:register_observer(observer)
subject:notifyObservers()
