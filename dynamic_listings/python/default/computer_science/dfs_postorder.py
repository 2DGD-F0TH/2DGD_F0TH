def dfs_traverse_postorder(n):
    # Step 1: We traverse the left subtree, using recursion
    if n.left is not None:
        dfs_traverse_postorder(n.left)
    # Step 2: We traverse the right subtre, using recursion
    if n.right is not None:
        dfs_traverse_postorder(n.right)
    # Step 3: Visit the node, in this case we print its value
    print(n.content)


if __name__ == "__main__":
    root = build_example_tree()
    dfs_traverse_postorder(root)
