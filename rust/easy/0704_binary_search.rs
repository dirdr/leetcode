impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0 as usize, nums.len());
        while (l < r) {
            let middle = l + (r - l) / 2;
            match target.cmp(&nums[middle]) {
                std::cmp::Ordering::Equal => return middle as i32,
                std::cmp::Ordering::Less => r = middle,
                std::cmp::Ordering::Greater => l = middle + 1,
            }
        }
        -1
    }
}
