 use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }
        
        let mut min_heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut visited = vec![vec![false; n]; m];

        min_heap.push((Reverse(0), 0, 0));
        visited[0][0] = true;
        let mut t = 0;
        while let Some((Reverse(time), row, col)) = min_heap.pop() {
            if row == m - 1 && col == n - 1 {
                return time;
            }  
            for (dr, dc) in &DIRS {
                let next_r = row as isize + dr;
                let next_c = col as isize + dc;
                if next_r < 0 || next_r >= m as isize || next_c < 0 || next_c >= n as isize {
                    continue;
                } 
                let (next_r, next_c) = (next_r as usize, next_c as usize);

                if visited[next_r][next_c] {
                    continue;
                }
                let mut next_time = time + 1;
                if next_time < grid[next_r][next_c] {
                    let wait_time = grid[next_r][next_c] - next_time;
                    next_time += wait_time + wait_time % 2;
                }
                
                min_heap.push((Reverse(next_time), next_r, next_c));
                visited[next_r][next_c] = true;
            }
        }
        -1
    }
}
