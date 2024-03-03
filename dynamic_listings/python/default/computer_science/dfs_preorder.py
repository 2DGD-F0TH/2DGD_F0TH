def dfs_traverse_preorder(n):
    # Step 1: Visit the node, in this case we print its value
    print(n.content)
    # Step 2: We traverse the left subtree, using recursion
    if (n.left is not None):
        dfs_traverse_preorder(n.left)
    # Step 3: We traverse the right subtre, using recursion
    if (n.right is not None):
        dfs_traverse_preorder(n.right)


if __name__ == "__main__":
    Node root = build_example_tree()
    dfs_traverse_preorder(root)
