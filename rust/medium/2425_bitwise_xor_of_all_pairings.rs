use std::collections::HashMap;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut freqs = HashMap::new();
        let (n, m) = (nums1.len(), nums2.len());
        for &el in &nums1 {
            freqs.entry(el).and_modify(|e| *e += m).or_insert(m);
        }
        for &el in &nums2 {
            freqs.entry(el).and_modify(|e| *e += n).or_insert(n);
        }
        println!("{:?}", freqs);
        freqs
            .iter()
            .filter(|(&k, &v)| v % 2 != 0)
            .map(|(k, _)| k)
            .fold(0, |acc, x| acc ^ x)
    }
}
