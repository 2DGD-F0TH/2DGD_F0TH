# #![allow(dead_code)]
# struct Node {
#     content: &'static str,
#     left: Option<Box<Node>>,
#     right: Option<Box<Node>>,
# }
impl Node {
    fn traverse_bfs(self) {
        // We will use a queue for this algorithm
        let mut q = std::collections::VecDeque::new();
        let root = Box::new(self);
        // First thing, we enqueue the root
        q.push_back(root);
        // Now comes the iterative part. This will keep going until
        // the tree is completely explored.
        while let Some(n) = q.pop_front() {
            // We enqueue its children, if they exist
            if let Some(left) = n.left {
                q.push_back(left);
            }
            if let Some(right) = n.right {
                q.push_back(right);
            }
            // Now we visit the current node
            print!("{}", n.content);
            // The loop will continue with the next node in the layer,
            // automatically start the next layer, or stop because there
            // are no more nodes to visit.
        }
    }
}
