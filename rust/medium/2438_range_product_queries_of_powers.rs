impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MODULO: i64 = 1_000_000_007;
        let mut powers = vec![];
        for i in 0..32 {
            if 1 & (n >> i) == 1 {
                powers.push(2_i32.pow(i as u32));
            }
        }
        queries.iter().map(|q| {
            let mut product = 1i64;
            for i in q[0] as usize..=q[1] as usize {
                product = (product * powers[i] as i64) % MODULO;
            }
            product as i32
        }).collect()
    }
}
