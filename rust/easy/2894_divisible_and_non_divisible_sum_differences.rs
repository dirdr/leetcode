impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let total = n * (n + 1) / 2;
        let k = n / m;
        let sum_div = m * k * (k + 1) / 2;
        (total - sum_div) - sum_div
    }
}
