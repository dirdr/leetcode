impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0 as usize, 0 as usize);
        let mut max = 0;
        while r < prices.len() {
            if prices[r] < prices[l] {
                l = r;
            } else {
                max = std::cmp::max(prices[r] - prices[l], max);
            }
            r += 1;
        }
        max
    }
}
