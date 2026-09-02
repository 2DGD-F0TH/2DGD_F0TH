# #![allow(dead_code)]
#[derive(Clone, PartialEq)]
struct Node {
    parent: Option<Box<Node>>, // This will be used to build the path
    g: f32,                    // The path cost value for the node
}
