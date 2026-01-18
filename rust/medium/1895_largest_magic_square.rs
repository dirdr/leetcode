impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        fn is_magic(grid: &[Vec<i32>], r: usize, c: usize, len: usize) -> bool {
            let (mut sumd1, mut sumd2) = (0, 0);
            for i in 0..len {
                sumd1 += grid[r + i][c + i];
                sumd2 += grid[r + i][c + len - 1 - i];
            }
            if sumd1 != sumd2 {
                return false;
            }
            for i in 0..len {
                let row_sum: i32 = (0..len).map(|j| grid[r + i][c + j]).sum();
                let col_sum: i32 = (0..len).map(|j| grid[r + j][c + i]).sum();

                if row_sum != sumd1 || col_sum != sumd1 {
                    return false;
                }
            }
            true
        }
        
        let (n, m) = (grid.len(), grid[0].len());
        let max_len = n.min(m);
        
        for len in (2..=max_len).rev() {
            for i in 0..=n - len {
                for j in 0..=m - len {
                    if is_magic(&grid, i, j, len) {
                        return len as i32;
                    }
                }
            }
        }
        1
    }
}
