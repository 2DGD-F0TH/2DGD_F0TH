class MyMemoizedObject:
    memory: dict[str, str] = {}   # Works with any hashable types

    def memoized_function(self, parameter: str) -> str:
        if parameter in self.memory:
            # If the result was calculated earlier, we can just return it
            return self.memory[parameter]
        # If the result has never been calculated we do so.
        # ...
        # Very complex and heavy calculations here
        # ...
        result: str = something_complex()
        # Now we save the result in our memory, so other calls with the same
        # parameter will be faster
        self.memory[parameter] = result
        return result
