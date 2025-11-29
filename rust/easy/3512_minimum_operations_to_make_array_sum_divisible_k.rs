impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let total = nums.iter().sum::<i32>();
        let mut curr = total;
        while curr % k != 0 {
            curr -= 1;
        }
        total - curr
    }
}
