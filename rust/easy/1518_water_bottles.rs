impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let (mut f, mut b, mut count) = (num_bottles, 0, 0);
        while f > 0 {
            count += f;
            b = f + (b % num_exchange);
            f = b / num_exchange;
        }
        count
    }
}
