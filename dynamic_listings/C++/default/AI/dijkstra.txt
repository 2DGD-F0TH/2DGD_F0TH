#include <vector>
#include <algorithm>

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
            if (std::find(openSet->begin(), openSet->end(), currentNode) != openSet->end()){
                // Check if this path is better
                float new_g = getPathCost(n, start);
                if (new_g < n->g){
                    // We found a better path from start to currentNode
                    n->parent = currentNode;
                    n->g = new_g;
                }
            }else{
                n->parent = currentNode;
                n->g = getPathCost(n, start);
                openSet->push_back(currentNode);
            }
        }
    }
    if (openSet->empty()){
        // We exhausted all possibilities
        break;
    }

    // Order openSet by g
    std::sort(openSet->begin(), openSet->end(), [](Node* i, Node* j){return (i->g < i->g);});
    // Since openset is ordered by g, the first element is the one with the lowest total cost
    currentNode = openset->front();
    openset->erase(openset->front());
    closedSet->push_back(currentNode);
} while(currentNode != end);
