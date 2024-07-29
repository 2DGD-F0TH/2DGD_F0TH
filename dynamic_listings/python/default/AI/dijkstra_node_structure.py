from typing import Self

class Node:
    parent: Self = None  # This will be used to build the path
    h: float = None  # The path cost value for the node
