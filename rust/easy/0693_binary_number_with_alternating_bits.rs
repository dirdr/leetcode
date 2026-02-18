impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let digits = n.ilog2() + 1;
        let mut prev = n & 1;
        for i in 1..digits {
            let bit = (n >> i) & 1;
            if bit == prev {
                return false;
            }
            prev = bit;
        }
        true
    }
}
