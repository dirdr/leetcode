use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut positions = HashMap::new();
        let (mut rows_count, mut cols_count) = (vec![0; m], vec![0; n]);
        for i in 0..m {
            for j in 0..n {
                positions.insert(mat[i][j], (i, j));
            }
        }
        for (i, el) in arr.iter().enumerate() {
            let &(r, c) = positions.get(&el).unwrap();
            rows_count[r] += 1;
            cols_count[c] += 1;
            if rows_count[r] == n || cols_count[c] == m {
                return i as i32;
            }
        }
        -1
    }
}
