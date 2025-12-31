use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        fn can_cross(map: &[Vec<i32>], row: usize, col: usize) -> bool {
            let mut queue = VecDeque::new();
            let mut visited = vec![vec![false; col]; row];
            
            for c in 0..col {
                if map[0][c] == 0 {
                    queue.push_back((0, c));
                    visited[0][c] = true;
                }
            }
            
            let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            
            while let Some((r, c)) = queue.pop_front() {
                if r == row - 1 {
                    return true;
                }
                
                for &(dr, dc) in &directions {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    
                    if nr >= 0 && nr < row as i32 && nc >= 0 && nc < col as i32 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        
                        if !visited[nr][nc] && map[nr][nc] == 0 {
                            visited[nr][nc] = true;
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
            
            false
        }
        
        let row = row as usize;
        let col = col as usize;
        let mut left = 0;
        let mut right = cells.len() - 1;
        let mut result = 0;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            let mut map = vec![vec![0; col]; row];
            
            for i in 0..mid {
                let r = cells[i][0] as usize - 1;
                let c = cells[i][1] as usize - 1;
                map[r][c] = 1;
            }
            
            if can_cross(&map, row, col) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result as i32
    }
}
