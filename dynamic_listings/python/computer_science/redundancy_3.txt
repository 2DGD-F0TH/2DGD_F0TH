def get_previous_node(lst, current_node):
    pointer = lst.head
    previous = None
    while (pointer != current_node):
        previous = pointer
        pointer = pointer.next
    return previous
