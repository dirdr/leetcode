impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        n * n + (n - 1) * (n - 1)
    }
}
