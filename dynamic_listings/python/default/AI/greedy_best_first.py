# We bootstrap the variables
openSet: list = []
closedSet: list = []
currentNode: Node = start
closedSet.append(currentNode)
while (currentNode != end):
    for n in currentNode.getAdjacentList():
        if n in closedSet:
            # We already analyzed this node, continue to next n
            continue
        n.parent = currentNode
        if n not in openSet:
            n.h = getHeuristics(n, end)  # Computes the value of n's h(x)
            openSet.append(n)
    if openSet.empty():
        # We exhausted all the possibilities
        break
    # Select a new "currentNode"
    openSet = sorted(openSet, key=lambda n: n.h)
    # Since openset is ordered by h(x) the first element is the one with the lowest h(x)
    currentNode = openSet[0]
    openSet.remove(currentNode)
    closedSet.append(currentNode)

if currentNode == end:
    # We reached the end and solved the path, we need to do a stack
    # reversal to find the path
    finalPath: list = []
    n = end
    while n is not None:
        finalPath.append(n)
        n = n.parent  # We use "parent" to run the found path backwards
else:
    # We cannot find a path between "start" and "end"
    pass
