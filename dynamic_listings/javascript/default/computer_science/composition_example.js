class Grinder{
    // A simple coffee grinder component
    grind(){
        // Pretend to grind some coffee
        console.log("Grinding coffee");
    }
}

class BrewingUnit{
    // A simple brewing unit component
    brew(){
        // Pretend to brew a good coffee
        console.log("Brewing your coffee");
    }
}

class CoffeeMachine{
    // A simple coffee machine, has a grinder and a brewing unit
    constructor(){
        this.grinder = new Grinder();
        this.brewer = new BrewingUnit();
    }

    make_coffee(){
        // Uses the brewing component and the grinder to make some fresh coffee
        this.grinder.grind();
        this.brewer.brew();
        console.log("Here's your fresh coffee!");
    }
}
