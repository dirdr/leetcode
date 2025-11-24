impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut rem = 0;
        nums.into_iter().map(|x| {
            rem = ((2 * rem) + x) % 5;
            rem == 0
        }).collect::<Vec<_>>()
    }
}
