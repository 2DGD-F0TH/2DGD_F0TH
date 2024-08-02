from typing import Self


class Node:
    """
    This is an example of a simple node structure for a tree.
    It can be used as root or any other node
    """

    def __init__(self, value: str):
        self.content: str = value
        self.left: Self = None
        self.right: Self = None


def build_example_tree() -> Node:
    # Let's build the example tree starting with the nodes
    A: Node = Node("A")
    B: Node = Node("B")
    C: Node = Node("C")
    D: Node = Node("D")
    E: Node = Node("E")
    F: Node = Node("F")
    G: Node = Node("G")
    H: Node = Node("H")
    I: Node = Node("I")
    # Now we connect the various components (the edges)
    B.left = A
    B.right = C
    F.left = E
    D.left = B
    D.right = F
    H.right = I
    G.left = D
    G.right = H
    # The tree is ready to be used, let's return the root (G)
    return G
