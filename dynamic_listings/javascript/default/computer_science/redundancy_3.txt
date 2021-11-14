function get_previous_node(lst, current_node){
    let pointer = lst.head;
    let previous = null;
    while (pointer != current_node){
        previous = pointer;
        pointer = pointer.next;
    }
    return previous;
}
