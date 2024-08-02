from typing import Self


class Node:
    parent: Self = None  # This will be used to build the path
    h: float = None  # The h(x) value for the node
