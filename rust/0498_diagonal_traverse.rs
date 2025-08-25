impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut diagonals: Vec<Vec<i32>> = vec![vec![]; m + n - 1];

        for i in 0..m {
            for j in 0..n {
                diagonals[i + j].push(mat[i][j]);
            }
        }

        let mut result = Vec::with_capacity(m * n);
        for (d, mut diag) in diagonals.into_iter().enumerate() {
            if d % 2 == 0 {
                diag.reverse();
            }
            result.extend(diag);
        }

        result
    }
}
