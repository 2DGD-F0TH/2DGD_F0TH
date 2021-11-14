// We bootstrap the variables
let openSet = [];
let closedSet = [];
let currentNode = start;
closedSet.push(currentNode);  // This will avoid re-analizing this node
do{
    for (const n of currentNode.getAdjacentList()){
        if (closedSet.includes(n)){
            // We already analyzed this node, continue to next n
            continue;
        }else{
            n.parent = currentNode;
            if (!openSet.includes(n)){
                n.h = getHeuristics(n, end);  // Computes the value of n's h(x)
                openSet.push(n);
            }
        }
    }
    if (openSet.length === 0){
        // We exhausted all possibilities
        break;
    }
    // Select a new "currentNode"
    // We order the openset by h, the first element will be the one with the lowest total cost
    openSet.sort((first, second) => {
        if (first.h < second.h){
            return -1;
        }
        if (first.h > second.h){
            return 1;
        }
        return 0;
    });
    // Shift pops and returns the first element of the array
    currentNode = openSet.shift();
    closedSet.push(currentNode);
}while (currentNode != end);

if (currentNode == end){
    /* We reached the end and solved the path, we need to do a stack
       reversal to find the path */
    let finalPath = [];
    let n = end;
    while (n != null){
        finalPath.push(n);
        n = n.parent;  // We use "parent" to run the found path backwards
    }
}else{
    // We cannot find a path between "start" and "end"
    // ... Behave accordingly
}
