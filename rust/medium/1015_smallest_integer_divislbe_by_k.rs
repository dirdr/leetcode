impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut rem = 0i32;
        for i in 1..=k as usize {
            rem = (rem * 10 + 1).rem_euclid(k);
            if rem == 0 {
                return i as i32;
            }
        }
        -1
    }
}
