impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut swap_needed = 0;
        let mut total_swaps = 0;
        for &byte in s.as_bytes().iter().rev() {
            if byte == b'0' {
                swap_needed += 1;
            } else {
                total_swaps += swap_needed;
            }
        }
        total_swaps
    }
}
