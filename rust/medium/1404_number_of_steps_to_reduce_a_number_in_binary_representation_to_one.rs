impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut steps = 0;
        let mut carry = 0;
        for b in s.as_bytes().iter().skip(1).rev() {
            let bit = (b - b'0') as i32;
            if bit + carry == 1 {
                steps += 2;
                carry = 1;
            } else {
                steps += 1;
            }
        }
        steps + carry
    }
}
