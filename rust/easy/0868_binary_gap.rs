impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut max = 0;
        let mut prev = -1;
        for i in 0..32 {
            let d = (n >> i) & 1;
            if d == 1 {
                if prev != -1 {
                    max = max.max(i - prev);
                }
                prev = i;
            }
        }
        max
    }
}
