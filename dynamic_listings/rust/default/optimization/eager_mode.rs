# #![allow(dead_code)]
struct EagerObject {
    halved_numbers: Vec<u32>,
}

impl EagerObject {
    pub fn new(numbers: &[u32]) -> Self {
        Self {
            // Prepares the halved numbers list
            halved_numbers: numbers.iter().map(|x| x / 2).collect(),
        }
    }

    pub fn object(&self, index: usize) -> u32 {
        // Returns the pre-calculated object at the requested index
        self.halved_numbers[index]
    }
}
