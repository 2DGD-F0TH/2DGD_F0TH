// Defines the abstract class/interface for the component pattern
trait Component {
    // Do nothing, this is an abstract class
    fn update(&self);
}

// Defines the concrete component number 1
struct ConcreteComponent1 {}

impl Component for ConcreteComponent1 {
    fn update(&self) {
        // Do Stuff
    }
}

// Defines the concrete component number 2
struct ConcreteComponent2 {
    list: Vec<Box<dyn Component>>,
}

impl Component for ConcreteComponent2 {
    fn update(&self) {
        for comp in &self.list {
            comp.update();
        }
        // Do Other Stuff
    }
}

struct Client {
    first_component: ConcreteComponent1,
    second_component: ConcreteComponent2,
}

impl Client {
    fn update(&self) {
        self.first_component.update();
        self.second_component.update();
    }
}
