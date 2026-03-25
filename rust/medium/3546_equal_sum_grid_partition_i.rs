impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let total_sum = grid.iter().flatten().map(|&e| e as i64).sum::<i64>();
        if total_sum % 2 != 0 {
            return false;
        }   
        let target = total_sum / 2;
        let mut row_sum = 0i64;
        for i in 0..m - 1 {
            for j in 0..n {
                row_sum += grid[i][j] as i64;
            }
            if row_sum == target {
                return true;
            }
        }
        
        let mut col_sum = 0i64;
        for j in 0..n - 1 {
            for i in 0..m {
                col_sum += grid[i][j] as i64;
            }
            if col_sum == target {
                return true;
            }
        }
        
        false
    }
}
