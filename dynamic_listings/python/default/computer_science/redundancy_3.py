def get_previous_node(lst: List, current_node: Node) -> Node:
    pointer: Node = lst.head
    previous: Node = None
    while pointer != current_node:
        previous = pointer
        pointer = pointer.next
    return previous
