function dfs_traverse_inorder(n){
    // Step 1: We traverse the left subtree, using recursion;
    if (n.left != null){
        dfs_traverse_inorder(n.left);
    }
    // Step 2: Visit the node, in this case we print its value
    console.log(n.content);
    // Step 3: We traverse the right subtre, using recursion;
    if (n.right != null){
        dfs_traverse_inorder(n.right);
    }
}

function main(){
    let root = build_example_tree();
    dfs_traverse_inorder(root);
}
