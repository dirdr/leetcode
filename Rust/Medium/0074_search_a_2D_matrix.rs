impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let (rows, columns) = (matrix.len(), matrix[0].len());
        if target < matrix[0][0] || target > matrix[rows - 1][columns - 1] {
            return false;
        }
        let (mut l, mut r) = (0 as usize, rows * columns - 1);
        while (l <= r) {
            let middle = l + (r - l) / 2;
            let row = middle / columns;
            let column = middle % columns;
            match matrix[row][column].cmp(&target) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => l = middle + 1,
                std::cmp::Ordering::Greater => r = middle - 1,
            }
        }
        false
    }
}
