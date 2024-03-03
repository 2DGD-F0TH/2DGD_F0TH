def dfs_traverse_inorder(n):
    # Step 1: We traverse the left subtree, using recursion
    if n.left is not None:
        dfs_traverse_inorder(n.left)
    # Step 2: Visit the node, in this case we print its value
    print(n.content)
    # Step 3: We traverse the right subtre, using recursion
    if n.right is not None:
        dfs_traverse_inorder(n.right)


if __name__ == "__main__":
    root = build_example_tree()
    dfs_traverse_inorder(root)
