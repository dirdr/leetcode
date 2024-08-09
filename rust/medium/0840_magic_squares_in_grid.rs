use std::collections::HashSet;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }
        let mut count = 0;
        for r in 1..grid.len() - 1 {
            for c in 1..grid[0].len() - 1 {
                if grid[r][c] == 5 {
                    let mut set = HashSet::new();
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            set.insert(grid[(r as isize + dr) as usize][(c as isize + dc) as usize]);
                        }
                    }  
                    if set.len() == 9 && set.iter().all(|&x| x >= 1 && x <= 9) {
                        let s1 = grid[r-1][c-1] + grid[r-1][c] + grid[r-1][c+1];
                        let s2 = grid[r][c-1] + grid[r][c] + grid[r][c+1];
                        let s3 = grid[r+1][c-1] + grid[r+1][c] + grid[r+1][c+1];
                        let s4 = grid[r-1][c-1] + grid[r][c-1] + grid[r+1][c-1];
                        let s5 = grid[r-1][c] + grid[r][c] + grid[r+1][c];
                        let s6 = grid[r-1][c+1] + grid[r][c+1] + grid[r+1][c+1];
                        let s7 = grid[r-1][c-1] + grid[r][c] + grid[r+1][c+1];
                        let s8 = grid[r-1][c+1] + grid[r][c] + grid[r+1][c-1]; 
                        if s1 == 15 && s2 == 15 && s3 == 15 && s4 == 15 && 
                           s5 == 15 && s6 == 15 && s7 == 15 && s8 == 15 {
                            count += 1;
                        }
                    }
                }
            }
        } 
        count
    }
}
