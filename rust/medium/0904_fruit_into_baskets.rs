use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut freqs = HashMap::new();
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..fruits.len() {
            *freqs.entry(fruits[right]).or_insert(0) += 1;

            while freqs.len() > 2 {
                let fruit = fruits[left];
                if let Some(count) = freqs.get_mut(&fruit) {
                    *count -= 1;
                    if *count == 0 {
                        freqs.remove(&fruit);
                    }
                }
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
