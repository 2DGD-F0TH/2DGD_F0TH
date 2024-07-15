#include <iostream>

class Grinder{
    // A simple coffee grinder component
public:
    void grind(){
        // Pretend to grind some coffee
        std::cout << "Grinding coffee" << std::endl;
    }
};

class BrewingUnit{
    // A simple brewing unit component
public:
    void brew(){
        // Pretend to brew a good coffee
        std::cout << "Brewing your coffee" << std::endl;
    }
};

class CoffeeMachine{
    // A simple coffee machine, has a grinder and a brewing unit
    Grinder grinder = Grinder();
    BrewingUnit brewer = BrewingUnit();

    void make_coffee(){
        // Uses the brewing component and the grinder to make some fresh coffee
        grinder.grind();
        brewer.brew();
        std::cout << "Here's your fresh coffee!" << std::endl;
    }
};
