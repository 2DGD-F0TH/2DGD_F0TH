# We bootstrap the variables
openSet: list = []
closedSet: list = []
currentNode: Node = start
closedSet.append(currentNode)
while (currentNode != end):
    for n in currentNode.getAdjacentList():
        if n in closedSet:
            continue
        if n in openSet:  # Check if this path is better
            new_g = getPathCost(n, start)
            if new_g < n.g:
                # We found a better path from start to currentNode
                n.parent = currentNode
                n.g = new_g
        else:
            n.parent = currentNode
            n.g = getPathCost(n, start)
            openSet.append(currentNode)

    if openSet.empty():
        # We exhausted all possibilities
        break
    openSet = sorted(openSet, key=lambda n: n.g)
    # Since openset is ordered by g, the first element is the one with the lowest path cost
    currentNode = openSet[0]
    openSet.remove(currentNode)
    closedSet.append(currentNode)

# Reconstruct the path like in the greedy "best-first" algorithm
