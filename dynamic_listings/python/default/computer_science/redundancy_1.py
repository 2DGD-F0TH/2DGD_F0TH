class List:
    node_list: list[Node] = None

    # ...

    def __len__(self):
        counter: int = 0
        node = self.node_list[0]
        while node:
            counter += 1
            node = node.next
        return counter
