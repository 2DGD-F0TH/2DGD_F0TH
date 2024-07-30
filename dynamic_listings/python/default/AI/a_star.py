# We bootstrap the variables
openSet: list[Node] = []
closedSet: list[Node] = []
currentNode: Node = start
closedSet.append(currentNode)
while (currentNode != end):
    for n in currentNode.getAdjacentList():
        if n in closedSet:
            continue
        if n in openSet:  # Check if this path is better
            new_g: float = getPathCost(n, start)
            if new_g < n.g:
                # We found a better path from start to currentNode
                n.parent = currentNode
                n.g = new_g
                n.f = n.g + n.h
        else:
            n.parent = currentNode
            n.g = getPathCost(n, start)
            n.h = getHeuristicCost(n, end)
            n.f = n.g + n.h
            openSet.append(currentNode)

    if not openSet:
        # We exhausted all possibilities
        break
    openSet = sorted(openSet, key=lambda n: n.f)
    # Since openset is ordered by f, the first element is the one with the lowest total cost
    currentNode = openSet[0]
    openSet.remove(currentNode)
    closedSet.append(currentNode)

# Reconstruct the path like in the greedy "best-first" algorithm
