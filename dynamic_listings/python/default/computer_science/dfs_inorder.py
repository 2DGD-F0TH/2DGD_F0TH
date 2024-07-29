def dfs_traverse_inorder(n: Node):
    # Step 1: We traverse the left subtree, using recursion
    if n.left:
        dfs_traverse_inorder(n.left)
    # Step 2: Visit the node, in this case we print its value
    print(n.content)
    # Step 3: We traverse the right subtre, using recursion
    if n.right:
        dfs_traverse_inorder(n.right)


if __name__ == "__main__":
    root: Node = build_example_tree()
    dfs_traverse_inorder(root)
