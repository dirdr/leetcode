use std::collections::HashSet;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut prefixes = HashSet::new();
        let mut max_length = 0;
        fn process_prefixes<F>(arr: &[i32], mut callback: F)
        where
            F: FnMut(i32),
        {
            for &el in arr {
                let mut pref = el;
                while pref >= 1 {
                    callback(pref);
                    pref /= 10;
                }
            }
        }
        process_prefixes(&arr1, |pref| {
            prefixes.insert(pref);
        });
        process_prefixes(&arr2, |pref| {
            if prefixes.contains(&pref) {
                let len = (pref as f64).log10() as i32 + 1;
                max_length = max_length.max(len);
            }
        });

        max_length
    }
}
