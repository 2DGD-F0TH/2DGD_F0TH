local function get_previous_node(lst, current_node)
    local pointer = lst.head;
    local previous = nil;
    while (pointer ~= current_node) do
        previous = pointer;
        pointer = pointer.next;
    end
    return previous;
end
