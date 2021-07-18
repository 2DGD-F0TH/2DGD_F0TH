class List:
    nodeList = None

    # ...

    def __len__(self):
        counter = 0
        nodeptr = self.nodeList
        while nodeptr:
            counter += 1
            nodeptr = nodeptr.next
        return counter
