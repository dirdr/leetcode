impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut answer = x as i64;
        let mut to_merge = n as i64 - 1;
        let mut current = 0;
        for i in 0..((i64::MAX as f32).log2() as i32 + 1) {
            if (answer >> i) & 1 == 0 {
                let bit = (to_merge >> current) & 1;
                answer |= (bit << i) as i64;
                current += 1;
            }
        }
        answer
    }
}
