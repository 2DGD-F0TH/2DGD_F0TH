class Component:
    # Defines the abstract class/interface for the component pattern
    def update(self):
        # Do nothing, this is an abstract class
        raise NotImplementedError()


class ConcreteComponent1(Component):
    # Defines the concrete component number 1
    def update(self):
        # Do Stuff
        pass


class ConcreteComponent2(Component):
    # Defines the concrete component number 2

    # The component can contain a list of other components that get updated
    component_list: list[Component] = []

    def update(self):
        for comp in self.component_list:
            comp.update()

        # Do Other Stuff


class Client:
    first_component = ConcreteComponent1()
    second_component = ConcreteComponent2()

    def update(self):
        # This is the Client's update function
        self.first_component.update()
        self.second_component.update()
