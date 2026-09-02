# #![allow(dead_code, unused)]
#[derive(Clone, PartialEq)]
struct Node {
    parent: Option<Box<Node>>, // This will be used to build the path
    h: f32,                    // The h(x) value for the node
}
