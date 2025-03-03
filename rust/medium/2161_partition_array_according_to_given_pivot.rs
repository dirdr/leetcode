impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let (mut smallers, mut greaters) = (vec![], vec![]);
        let mut equal = 0;
        let mut ans = Vec::with_capacity(nums.len());
        for &el in &nums {
            match el.cmp(&pivot) {
                std::cmp::Ordering::Greater => greaters.push(el),
                std::cmp::Ordering::Equal => equal += 1,
                std::cmp::Ordering::Less => smallers.push(el),
            }
        }
        ans.extend_from_slice(&smallers);
        ans.extend_from_slice(&vec![pivot; equal]);
        ans.extend_from_slice(&greaters);
        ans
    }
}
