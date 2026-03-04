impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut specials = 0;
        let (m, n) = (mat.len(), mat[0].len());
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    continue;
                }
                let mut valid = true;
                for k in 0..m {
                    if mat[k][j] == 1 && k != i {
                        valid = false;
                    }
                }
                for k in 0..n {
                    if mat[i][k] == 1 && k != j {
                        valid = false;
                    }
                }
                if valid {
                    specials += 1;
                }
            }
        }
        specials
    }
}
