impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut entry = prices[0];
        let mut max_profit = 0;
        for i in 1..prices.len() {
            if prices[i] < prices[i - 1] {
                max_profit += prices[i - 1] - entry;
                entry = prices[i];
            } else {
                entry = entry.min(prices[i]);
            }
        }
        max_profit += prices[prices.len() - 1] - entry;
        max_profit
    }
}
