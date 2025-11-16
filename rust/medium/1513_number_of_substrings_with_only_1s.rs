impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const M: i64 = 1_000_000_007;

        s.as_bytes()
            .chunk_by(|a, b| a == b)
            .filter(|chunk| chunk[0] == b'1')
            .map(|chunk| {
                let k = chunk.len() as i64;
                k * (k + 1) / 2 % M
            })
            .fold(0i64, |acc, x| (acc + x) % M) as i32
    }
}
