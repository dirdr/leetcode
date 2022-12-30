use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::new();
        for i in 1..=n {
            let mut number = i;
            let mut sum = 0;
            while number > 0 {
                let digit = number % 10;
                sum += digit;
                number = number / 10;
            }
            map.entry(sum).and_modify(|e| {*e += 1}).or_insert(1);
        }
        let mut max_size = map
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(_, v)| v)
            .unwrap();
        map
            .iter()
            .filter(|(_, value)| *value == max_size)
            .count() as i32
    }
}
