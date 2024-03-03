class Node:
    """
    This is an example of a simple node structure for a tree.
    It can be used as root or any other node
    """

    def __init__(self, value):
        self.content = value
        self.left = None
        self.right = None


def build_example_tree():
    # Let's build the example tree starting with the nodes
    A = Node("A")
    B = Node("B")
    C = Node("C")
    D = Node("D")
    E = Node("E")
    F = Node("F")
    G = Node("G")
    H = Node("H")
    I = Node("I")
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
