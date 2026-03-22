impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let (m, n) = (mat.len(), mat[0].len());
        for time in 0..4 {
            let mut new = mat.clone();
            for i in 0..m {
                for j in 0..n {
                    new[n - j - 1][i] = mat[i][j]
                }
            }
            std::mem::swap(&mut mat, &mut new);
            if mat == target {
                return true;
            }
        }
        false
    }
}
