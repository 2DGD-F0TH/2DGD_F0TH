trait Strategy {
    // This class defines the strategy interface the client will refer to

    // This algorithm will be implemented by the subclasses
    fn algorithm(&self);
}

struct ConcreteStrategy1;

impl Strategy for ConcreteStrategy1 {
    fn algorithm(&self) {
        // Real implementation of the algorithm
        // DO STUFF
    }
}

struct ConcreteStrategy2;

impl Strategy for ConcreteStrategy2 {
    fn algorithm(&self) {
        // Real implementation of the algorithm
        // DO STUFF SLIGHTLY DIFFERENTLY
    }
}

// Example Usage
fn main() {
#   let condition = true;
    let to_execute = if condition {
        Box::new(&ConcreteStrategy1 as &dyn Strategy)
    } else {
        Box::new(&ConcreteStrategy2 as &dyn Strategy)
    };
    to_execute.algorithm(); // This will execute 1 or 2 depending on "condition"
}
