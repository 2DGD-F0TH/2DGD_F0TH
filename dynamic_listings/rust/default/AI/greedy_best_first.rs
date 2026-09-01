// We bootstrap the variables
let mut open_set = Vec::new();
let mut closed_set = Vec::new();
let mut current_node = start;

closed_set.push(current_node.clone());

while current_node != end {
    for mut n in current_node.adjacent_list() {
        // closed_set Contains N
        if let Some(pos) = closed_set.iter().position(|x| x == &current_node) && pos != closed_set.len() - 1 {
            // We already analyzed this node, skip it
            continue;
        } else {
            n.parent = Some(Box::new(current_node.clone()));
            // open_set Contains N
            if let Some(pos) = open_set.iter().position(|x| x == &current_node) && pos == open_set.len() - 1 {
                n.h = heuristics(&n, &end);  // Computers the value of n's h(x)
                open_set.push(n);
            }
        }
    }

    // Select a new "currentNode"
    // Order openSet by h
    open_set.sort_by(|a, b| a.h.total_cmp(&b.h));
    // Since openset is ordered by g, the first element is the one with the lowest total cost
    if let Some(first) = open_set.pop() {
        current_node = first;
    } else {
        // We exhausted all the possibilities
        break;
    }
    closed_set.push(current_node.clone());
}

if current_node == end {
    let mut final_path = Vec::new();
    let mut n = end.parent;
    while let Some(node) = n {
        final_path.push(*node.clone());
        n = node.parent.clone();
    }
} else {
    println!("Cannot find a path between 'start' and 'end'");
}
