# #![allow(dead_code)]
use std::collections::BTreeMap;

struct MyMemoizedObject {
    memory: BTreeMap<String, String>,
}

impl MyMemoizedObject {
    fn memoized_function(&mut self, parameter: String) -> String {
        if let Some(result) = self.memory.get(&parameter) {
            // If the result was calculated earlier, we can just return it
            return result.clone();
        }
        // If the result has never been calculated we do so.
        // ...
        // Very complex and heavy calculations here
        // ...
#       let something_complex = String::new();
        let result = something_complex;
        // Now we save the result in our memory, so other calls with the same parameter will be faster
        self.memory.insert(parameter, result.clone());
        result
    }
}
