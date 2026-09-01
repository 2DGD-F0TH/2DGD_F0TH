struct List {
    node_list: Vec<Node>,
    length: usize,
    // ...
}

impl List {
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn add_item(&mut self, node: &Node) {
        // ... Normal operation ...
        // ...
        // We update our length counter
        self.length = self.length + 1;
    }

    pub fn remove_item(&mut self, node: &Node) {
        // ... Normal removal operation ...
        // ...
        // We update our length counter
        self.length = self.length - 1;
    }

    pub fn clear(&mut self) {
        // ... Normal clear operation ...
        // ...
        // We clear the length too
        self.length = 0;
    }
    // ...
}
