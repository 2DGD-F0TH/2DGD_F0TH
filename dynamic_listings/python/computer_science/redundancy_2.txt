class List:
    nodeList = None
    length = 0

    # ...

    def __len__(self):
        return self.length

    def addItem(self, node):
        # ... Normal operation ...
        # ...
        # We update our length counter
        self.length += 1

    def removeItem(self, node):
        # ... Normal removal operation ...
        # ...
        # We update our length counter
        self.length -= 1

    def clear(self):
        # ... Normal clear operation ...
        # ...
        # We clear the length too
        self.length = 0

    # ...
