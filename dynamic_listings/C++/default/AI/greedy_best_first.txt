#include <vector>
#include <algorithm>
#include <iostream>

// We bootstrap the variables
std::vector<Node*> openSet* = new std::vector<Node*>();
std::vector<Node*> closedSet* = new std::vector<Node*>();
Node* currentNode = start;

closedSet->push_back(currentNode);

do{
    for (Node* n : currentNode->getAdjacentList()){
        // ClosedSet Contains N
        if (std::find(closedSet->begin(), closedSet->end(), currentNode) != closedSet->end()){
            // We already analyzed this node, skip it
            continue;
        }else{
            n->parent = currentNode;
            // OpenSet Contains N
            if (std::find(openSet->begin(), openSet->end(), currentNode) == openSet->end()){
                n->h = getHeuristics(n, end);  // Computers the value of n's h(x)
                openSet->push_back(n);
            }
        }
    }

    if (openSet->empty()){
        // We exhausted all the possibilities
        break;
    }

    // Select a new "currentNode"
    // Order openSet by h
    std::sort(openSet->begin(), openSet->end(), [](Node* i, Node* j){return (i->h < i->h);});
    // Since openset is ordered by g, the first element is the one with the lowest total cost
    currentNode = openset->front();
    openset->erase(openset->front());
    closedSet->push_back(currentNode);
}while(currentNode != end);

if (currentNode == end){
    std::vector<Node*> finalPath* = new std::vector<Node*>();
    Node* n = end;
    while(n != nullptr){
        finalPath->push_back(n);
        n = n->parent;
    }
}else{
    std::cout << "Cannot find a path between 'start' and 'end'" << std::endl;
}
