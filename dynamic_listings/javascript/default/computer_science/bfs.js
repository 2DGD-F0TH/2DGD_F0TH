function traverse_bfs(root){
    // We will use a queue for this algorithm
    let q = [];
    // First thing, we enqueue the root
    q.push(root);
    // Now comes the iterative part. This will keep going until
    // the tree is completely explored.
    while (q.length != 0){
        // We take the first node in the queue
        let n = q.shift();
        // We enqueue its children, if they exist
        if (n.left != null){
            q.push(n.left);
        }
        if (n.right != null){
            q.push(n.right);
        }
        // Now we visit the current node
        console.log(n.content);
        // The loop will continue with the next node in the layer,
        // automatically start the next layer, or stop because there
        // are no more nodes to visit.
    }
}
