use std::collections::{HashMap, BTreeSet};

struct NumberContainers {
    index_to_num: HashMap<i32, i32>,
    num_to_indices: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            index_to_num: HashMap::new(),
            num_to_indices: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_num) = self.index_to_num.get(&index) {
            if let Some(indices) = self.num_to_indices.get_mut(&old_num) {
                indices.remove(&index);
                if indices.is_empty() {
                    self.num_to_indices.remove(&old_num);
                }
            }
        }
        
        self.index_to_num.insert(index, number);
        
        self.num_to_indices
            .entry(number)
            .or_insert_with(BTreeSet::new)
            .insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        self.num_to_indices
            .get(&number)
            .and_then(|indices| indices.first())
            .copied()
            .unwrap_or(-1)
    }
}
