#include <queue>
#include <iostream>

void traverse_bfs(Node* root){
    // We will use a queue for this algorithm
    std::queue<Node*> q = std::queue<Node*>();
    // First thing, we enqueue the root
    q.push(root);
    // Now comes the iterative part. This will keep going until
    // the tree is completely explored.
    while (q.size() != 0){
        // We take the first node in the queue
        Node* n = q.front();
        q.pop();
        // We enqueue its children, if they exist
        if (n->left != nullptr){
            q.push(n->left);
        }
        if (n->right != nullptr){
            q.push(n->right);
        }
        // Now we visit the current node
        std::cout << n->content;
        // The loop will continue with the next node in the layer,
        // automatically start the next layer, or stop because there
        // are no more nodes to visit.
    }
};
