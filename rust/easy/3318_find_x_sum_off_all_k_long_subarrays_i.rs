use std::collections::HashMap;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut map = HashMap::new();
        let mut l = 0;
        let mut answer = vec![];
        for r in 0..n {
            while r - l + 1 > k  {
                map.entry(nums[l]).and_modify(|e| *e -= 1);
                l += 1;
            }
            map.entry(nums[r]).and_modify(|e| *e += 1).or_insert(1);
            if r - l + 1 == k {
                let mut x_sum = map.iter().collect::<Vec<_>>();
                x_sum.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));
                let x_sum = x_sum.iter().take(x as usize).map(|&(k, v)| k * v).sum::<i32>();
                answer.push(x_sum);
            }
        }
        answer
    }
}
