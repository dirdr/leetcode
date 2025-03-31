impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prefix = 0;
        let mut max = i32::MIN;
        for &el in &nums {
            if prefix <= 0 {
                prefix = 0
            }
            prefix += el;
            max = max.max(prefix);
        }
        max
    }
}
