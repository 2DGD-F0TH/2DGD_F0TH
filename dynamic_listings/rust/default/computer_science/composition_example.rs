# #![allow(dead_code)]
// A simple coffee grinder component
struct Grinder;

impl Grinder {
    fn grind(&self) {
        // Pretend to grind some coffee
        println!("Grinding coffee");
    }
}

// A simple brewing unit component
struct BrewingUnit;

impl BrewingUnit {
    fn brew(&self) {
        // Pretend to brew a good coffee
        println!("Brewing your coffee");
    }
}

struct CoffeeMachine {
    grinder: Grinder,
    brewer: BrewingUnit,
}

impl CoffeeMachine {
    fn new() -> Self {
        Self {
            grinder: Grinder,
            brewer: BrewingUnit,
        }
    }

    fn make_coffee(&self) {
        self.grinder.grind();
        self.brewer.brew();
        println!("Here's your fresh coffee!");
    }
}
