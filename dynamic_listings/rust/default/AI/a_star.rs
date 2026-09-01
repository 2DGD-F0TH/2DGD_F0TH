let mut open_set = Vec::new();
let mut closed_set = Vec::new();
let mut current_node = start.clone();

closed_set.push(current_node.clone());

while current_node != end {
    for mut n in current_node.adjacent_list() {
        // closed_set Contains N
        if let Some(pos) = closed_set.iter().position(|x| x == &current_node) && pos != closed_set.len() - 1 {
            // We already analyzed this node, skip it
            continue;
        } else {
            if let Some(pos) = open_set.iter().position(|x| x == &current_node) && pos != open_set.len() - 1 {
                let new_g = path_cost(&n, &start);
                if new_g < n.g {
                    // We found a better path from start to currentNode
                    n.parent = Some(Box::new(current_node.clone()));
                    n.g = new_g;
                    n.f = n.g + n.h;
                }
            } else {
                n.parent = Some(Box::new(current_node.clone()));
                n.g = path_cost(&n, &start);
                n.h = heuristic_cost(&n, &end);
                n.f = n.g + n.h;
                open_set.push(current_node.clone());
            }
        }

        // Order openSet by f
        open_set.sort_by(|a, b| a.f.total_cmp(&b.f));
        // Since openset is ordered by f, the first element is the one with the lowest total cost
        if let Some(first) = open_set.pop() {
            current_node = first;
        } else {
            // We exhausted all the possibilities
            break;
        }
        closed_set.push(current_node.clone());
    }
}
