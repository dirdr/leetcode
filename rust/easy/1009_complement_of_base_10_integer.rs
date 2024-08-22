impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
       let mut num = n;
        for i in 0..(num as f64).log2().floor() as u32 + 1 {
            num ^= (1 << i);
        }
        num 
    }
}
