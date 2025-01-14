use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut n = a.len();
        let mut common = HashSet::new();
        let mut answer = Vec::with_capacity(n);
        let mut count = 0;
        for i in 0..n {
            if !common.insert(a[i]) {
                count += 1;
            }
            if !common.insert(b[i]) {
                count += 1;
            }
            answer.push(count);
        }
        answer
    }
}
