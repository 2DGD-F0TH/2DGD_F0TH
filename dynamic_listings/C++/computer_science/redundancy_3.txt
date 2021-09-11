Node* get_previous_node(List* lst, Node* current_node){
    Node* pointer = lst->head;
    Node* previous = nullptr;
    while (pointer != current_node){
        previous = pointer;
        pointer = pointer->next;
    }
    return previous;
}
