# struct Node {
#     content: &'static str,
#     left: Option<Box<Node>>,
#     right: Option<Box<Node>>,
# }
# fn build_example_tree() -> Node { todo!() }
impl Node {
    fn traverse_inorder(&self) {
        // Step 1: We traverse the left subtree, using recursion;
        if let Some(ref left) = self.left {
            left.traverse_inorder();
        }
        // Step 2: Visit the node, in this case we print its value
        print!("{}", self.content);
        // Step 3: We traverse the right subtre, using recursion;
        if let Some(ref right) = self.right {
            right.traverse_inorder();
        }
    }
}

fn main() {
    let root = build_example_tree();
    root.traverse_inorder();
}
