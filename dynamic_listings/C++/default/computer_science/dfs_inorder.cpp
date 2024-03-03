#include <iostream>

void dfs_traverse_inorder(Node n){
    // Step 1: We traverse the left subtree, using recursion;
    if (n.left != nullptr){
        dfs_traverse_inorder(*n.left);
    }
    // Step 2: Visit the node, in this case we print its value
    std::cout << n.content;
    // Step 3: We traverse the right subtre, using recursion;
    if (n.right != nullptr){
        dfs_traverse_inorder(*n.right);
    }
}

int main(){
    Node root = build_example_tree();
    dfs_traverse_inorder(root);
    return 0;
}
