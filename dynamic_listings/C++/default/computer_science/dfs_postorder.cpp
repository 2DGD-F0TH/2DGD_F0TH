#include <iostream>

void dfs_traverse_postorder(Node n){
    // Step 1: We traverse the left subtree, using recursion;
    if (n.left != nullptr){
        dfs_traverse_postorder(*n.left);
    }
    // Step 2: We traverse the right subtre, using recursion;
    if (n.right != nullptr){
        dfs_traverse_postorder(*n.right);
    }
    // Step 3: Visit the node, in this case we print its value
    std::cout << n.content;
}

int main(){
    Node root = build_example_tree();
    dfs_traverse_postorder(root);
    return 0;
}
