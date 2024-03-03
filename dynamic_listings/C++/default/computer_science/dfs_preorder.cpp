#include <iostream>

void dfs_traverse_preorder(Node n){
    // Step 1: Visit the node, in this case we print its value
    std::cout << n.content;
    // Step 2: We traverse the left subtree, using recursion;
    if (n.left != nullptr){
        dfs_traverse_preorder(*n.left);
    }
    // Step 3: We traverse the right subtre, using recursion;
    if (n.right != nullptr){
        dfs_traverse_preorder(*n.right);
    }
}

int main(){
    Node root = build_example_tree();
    dfs_traverse_preorder(root);
    return 0;
}
