impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let special_predicate = |a, b| (a % 2 == 0 && b % 2 != 0) || (a % 2 != 0 && b % 2 == 0);
        nums.windows(2).all(|w| special_predicate(w[0], w[1]))
    }
}
