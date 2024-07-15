class List:
    node_list: list[Node] = None
    length: int = None

    # ...

    def __len__(self) -> int:
        return self.length

    def add_item(self, node: Node) -> None:
        # ... Normal operation ...
        # ...
        # We update our length counter
        self.length += 1

    def remove_item(self, node: Node) -> None:
        # ... Normal removal operation ...
        # ...
        # We update our length counter
        self.length -= 1

    def clear(self) -> None:
        # ... Normal clear operation ...
        # ...
        # We clear the length too
        self.length = 0

    # ...
