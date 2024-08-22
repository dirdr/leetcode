impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut num = num;
        for i in 0..(num as f64).log2().floor() as u32 + 1 {
            num ^= (1 << i);
        }
        num
    }
}
