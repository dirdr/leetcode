impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::with_capacity(prices.len());
        for i in 0..prices.len() {
            let mut discount = false;
            for j in i + 1..prices.len() {
                if prices[j] <= prices[i] {
                    answer.push(prices[i] - prices[j]);
                    discount = true;
                    break;
                }
            }
            if !discount {
                answer.push(prices[i]);
            }
        }
        answer
    }
}
