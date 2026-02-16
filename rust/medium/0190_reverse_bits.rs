impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut answer = 0;
        for i in 0..32 {
            answer |= ((n >> (31 - i)) & 1) << i;
        }
        answer
    }
}
