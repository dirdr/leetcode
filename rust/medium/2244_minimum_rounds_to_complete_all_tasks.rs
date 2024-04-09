use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        if tasks.len() == 1 {return -1}
        let mut map = HashMap::new();
        tasks.iter().for_each(|el| {
            map.entry(el).and_modify(|e| *e += 1).or_insert(1);
        });
        if *map.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1 == 1 {return -1}
        let mut result = 0;
        map.iter()
            .map(|(key, value)| (value + 2) / 3)
            .sum()
    }
}
