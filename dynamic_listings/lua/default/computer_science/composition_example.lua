-- A simple coffee grinder component
Grinder = {}

function Grinder:new(o)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end

function Grinder:grind()
    -- Pretend to grind some coffee
    print("Grinding coffee")
end

BrewingUnit = {}

-- A simple brewing unit component
function BrewingUnit:new(o)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end

function BrewingUnit:brew()
    -- Pretend to brew a good coffee
    print("Brewing your coffee")
end

-- A simple coffee machine, has a grinder and a brewing unit
CoffeeMachine = {}

function CoffeeMachine:new(o)
    o = o or {
        grinder = Grinder:new(),
        brewer = BrewingUnit:new()
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function CoffeeMachine:make_coffee()
    -- Uses the brewing component and the grinder to make some fresh coffee
    self.grinder.grind()
    self.brewer.brew()
    print("Here's your fresh coffee!")
end
