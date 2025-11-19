impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, original: i32) -> i32 {
        nums.sort();
        let mut current = original;
        for &el in &nums {
            if current == el {
                current = (current << 1);
            }
        }
        current
    }
}
