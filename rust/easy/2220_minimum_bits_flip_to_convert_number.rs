impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        if start == 0 && goal == 0 {
            return 0;
        }
        let bits = std::cmp::max(start, goal).ilog2() + 1;
        let mut count = 0;
        for i in 0..bits {
            if (start >> i) & 1 ^ (goal >> i) & 1 == 1 {
                count += 1;
            }
        }
        count
    }
}
