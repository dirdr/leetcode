impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut current = 0;
        let mut i = 0;
        while i < bits.len() {
            if bits[i] == 0 {
                i += 1;
                current = 0;
            } else {
                i += 2;
                current = 1;
            }
        }
        current == 0
    }
}
