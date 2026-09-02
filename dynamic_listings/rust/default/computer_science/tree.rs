# #![allow(dead_code)]
/*
 * This is an example of a simple node structure for a tree.
 * It can be used as root or any other node
 */
struct Node {
    content: &'static str,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(content: &'static str) -> Self {
        Self {
            content,
            left: None,
            right: None,
        }
    }
}

fn build_example_tree() -> Node {
    // Let's build the example tree; starting with the nodes
    let a = Node::new("A");
    let mut b = Node::new("B");
    let c = Node::new("C");
    let mut d = Node::new("D");
    let e = Node::new("E");
    let mut f = Node::new("F");
    let mut g = Node::new("G");
    let mut h = Node::new("H");
    let i = Node::new("I");
    // Now we connect the various components (the edges)
    b.left = Some(Box::new(a));
    b.right = Some(Box::new(c));
    f.left = Some(Box::new(e));
    d.left = Some(Box::new(b));
    d.right = Some(Box::new(f));
    h.right = Some(Box::new(i));
    g.left = Some(Box::new(d));
    g.right = Some(Box::new(h));
    // The tree is ready to be used, let's return the root (g)
    g
}
