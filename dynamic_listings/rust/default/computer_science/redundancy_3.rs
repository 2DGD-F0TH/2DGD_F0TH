fn previous_node<'a>(lst: &'a List, current_node: &Box<Node>) -> Option<&'a Box<Node>> {
    let mut pointer = &lst.head;
    let mut previous = None;
    while pointer != current_node {
        previous = Some(pointer);
        pointer = match pointer.next {
            Some(ref next) => next,
            None => break,
        }
    }
    previous
}
