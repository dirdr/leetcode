impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in (i + 1)..n {
                (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
            }
        }
        for i in 0..n {
            matrix[i] = matrix[i].iter()
                .cloned()
                .rev()
                .collect::<Vec<_>>();
        }
    }
}
