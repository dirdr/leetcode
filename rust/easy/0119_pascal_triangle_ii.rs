impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut dp = vec![vec![1; row_index + 1]; row_index + 1];
        let mut row_size = 3;
        for i in 2..=row_index {
            for j in 1..row_size - 1 {
                dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
            }
            row_size += 1;
        }
        dp[row_index].clone()
    }
}
