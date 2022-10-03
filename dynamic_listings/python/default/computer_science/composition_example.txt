class Grinder:
    # A simple coffee grinder component
    def grind(self):
        # Pretend to grind some coffee
        print("Grinding coffee")


class BrewingUnit:
    # A simple brewing unit component
    def brew(self):
        # Pretend to brew a good coffee
        print("Brewing your coffee")


class CoffeeMachine:
    # A simple coffee machine, has a grinder and a brewing unit
    def __init__(self):
        # Constructor
        self.grinder: Grinder = Grinder()
        self.brewer: BrewingUnit = BrewingUnit()

    def make_coffee(self):
        # Uses the brewing component and the grinder to make some fresh coffee
        self.grinder.grind()
        self.brewer.brew()
        print("Here's your fresh coffee!")
