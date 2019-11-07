class Strategy:
    # This class defines the strategy interface the client will refer to

    def algorithm(self):
        # This algorithm will be implemented by the subclasses
        raise NotImplementedError("You should implement this")


class ConcreteStrategy1(Strategy):
    def algorithm():
        # Real implementation of the algorithm
        # DO STUFF
        pass


class ConcreteStrategy2(Strategy):
    def algorithm():
        # Real implementation of the algorithm
        # DO STUFF SLIGHTLY DIFFERENTLY
        pass


# Example Usage
def main():
    to_execute = None
    if condition:
        to_execute = ConcreteStrategy1()
    else:
        to_execute = ConcreteStrategy2()
    to_execute.algorithm()  # This will execute 1 or 2 depending on "condition"
