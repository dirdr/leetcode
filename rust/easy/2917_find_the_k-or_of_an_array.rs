use std::collections::HashMap;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for el in nums {
            for shift in 0..32 {
                if el & (1 << shift) != 0 {
                    map.entry(shift).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
        let mut result: i32 = 0;
        for (i, &el) in map.iter() {
            if el >= k {
                result = result | (1 << i);
            }
        }
        result
    }
}
