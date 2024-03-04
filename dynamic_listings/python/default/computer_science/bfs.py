def traverse_bfs(root):
    # We will use a queue for this algorithm (simulated by a list)
    q = []
    # First thing, we enqueue the root
    q.append(root)
    # Now comes the iterative part. This will keep going until
    # the tree is completely explored.
    while len(q) != 0:
        # We take the first node in the queue
        n = q.pop(0)
        # We enqueue its children, if they exist
        if n.left is not None:
            q.append(n.left)
        if n.right is not None:
            q.append(n.right)
        # Now we visit the current node
        print(n.content)
        # The loop will continue with the next node in the layer,
        # automatically start the next layer, or stop because there
        # are no more nodes to visit.
