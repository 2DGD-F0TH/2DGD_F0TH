impl Node {
    fn traverse_postorder(&self) {
        // Step 1: We traverse the left subtree, using recursion;
        if let Some(ref left) = self.left {
            left.traverse_postorder();
        }
        // Step 2: We traverse the right subtre, using recursion;
        if let Some(ref right) = self.right {
            right.traverse_postorder();
        }
        // Step 3: Visit the node, in this case we print its value
        print!("{}", self.content);
    }
}

fn main() {
    let root = build_example_tree();
    root.traverse_postorder();
}
