impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut total = 0;
        let mut neg = 0;
        let mut min_abs = i32::MAX;
        for i in 0..m {
            for j in 0..n {
                min_abs = min_abs.min(matrix[i][j].abs());
                total += matrix[i][j].abs() as i64;
                if matrix[i][j] < 0 {
                    neg += 1;
                }
            }
        }
        if neg % 2 == 0 {
            total
        } else {
            total - min_abs as i64 - min_abs as i64
        }
    }
}
