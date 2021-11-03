class Component{
    // Defines the abstract class/interface for the component pattern
    constructor(){
        // In javascript there are no real facilities for abstract classes
        // So we need to play around
        if (new.target == Handler){
            // The new.target changes in derivative classes
            throw new TypeError("Cannot construct abstract objects")
        }
    }

    update(){
        // Do nothing, this is an abstract class
    }
}

class ConcreteComponent1 extends Component{
    // Defines the concrete component number 1
    update(){
        // Do Stuff
    }
}

class ConcreteComponent2 extends Component{
    // Defines the concrete component number 2

    // The component can contain a list of other components that get updated
    constructor(){
        super();
        this.list = [component1, component2, ..., component9];
    }

    update(){
        for (const component in this.list){
            component.update();
        }

        // Do Other Stuff
    }
}

class Client{
    constructor(){
        this.first_component = new ConcreteComponent1();
        this.second_component = new ConcreteComponent2();
    }

    update(){
        // This is the Client's update function
        this.first_component.update();
        this.second_component.update();
    }
}
