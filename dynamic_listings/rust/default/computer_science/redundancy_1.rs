# #![allow(dead_code)]
#
# struct Node;
struct List {
    node_list: Vec<Node>,
    // ...
}

impl List {
    pub fn length(&self) -> usize {
        let mut counter = 0;
        let mut node = self.node_list.iter();
        while node.next().is_some() {
            counter = counter + 1;
        }
        counter
    }
}
