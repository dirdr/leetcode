use std::{collections::HashMap};

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    map: HashMap<i32, usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for &el in &nums2 {
            *map.entry(el).or_insert(0) += 1;
        }
        Self {
            nums1,
            nums2,
            map,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let old = self.nums2[index as usize];
        let new = old + val;
        *self.map.entry(old).or_insert(0) -= 1;
        *self.map.entry(new).or_insert(0) += 1;
        self.nums2[index as usize] += val;
    }

    fn count(&self, tot: i32) -> i32 {
        self.nums1.iter()
            .map(|e| *self.map.get(&(tot - e))
            .unwrap_or(&0) as i32)
            .sum::<i32>()
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
