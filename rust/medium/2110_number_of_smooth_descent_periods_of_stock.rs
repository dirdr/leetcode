impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut streak = 1;
        let mut answer = streak;
        for i in 1..prices.len() {
            if prices[i - 1] - prices[i] == 1 {
                streak += 1;
            } else {
                streak = 1;
            }
            answer += streak;
        }
        answer
    }
}
