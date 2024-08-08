impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![r_start, c_start]];
        let mut directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut p = vec![r_start, c_start];
        let mut i = 0;
        loop {
            let dir_offset = directions[i % 4];
            for d in 0..((i + 2) / 2) {
                p[0] += dir_offset.0;
                p[1] += dir_offset.1;
                if p[0] >= 0 && p[0] < rows && p[1] >= 0 && p[1] < cols {
                    result.push(p.clone());
                }
            }
            if result.len() == (rows * cols) as usize {
                break;
            }
            i += 1;
        }
        result
    }
}
