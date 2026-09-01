struct LazyObject {
    numbers_reference: Vec<u32>,
}

impl LazyObject {
    pub fn new(numbers: &[u32]) -> Self {
        // Saves the original list (possibly as a reference)
        Self {
            numbers_reference: numbers.to_vec(),
        }
    }

    pub fn object(&self, index: usize) -> u32 {
        // Calculates the halved number on-demand
        self.numbers_reference[index] / 2
    }
}
