class MyMemoizedObject:
    memory = {}

    def memoizedfunction(self, parameter):
        if parameter is self.memory.keys():
            # If the result was calculated earlier, we can just return it
            return self.memory[parameter]
        # If the result has never been calculated we do so.
        # ...
        # Very complex and heavy calculations here
        # ...
        result = something_complex
        # Now we save the result in our memory, so other calls with the same
        # parameter will be faster
        self.memory[parameter] = result
        return result
