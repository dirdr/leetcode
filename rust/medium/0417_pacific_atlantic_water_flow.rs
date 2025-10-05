use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        
        for i in 0..heights.len() {
            for j in 0..heights[0].len() {
                if Self::can_reach_both_oceans(i, j, &heights) {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }
    
    fn can_reach_both_oceans(row: usize, col: usize, heights: &[Vec<i32>]) -> bool {
        let (mut pacific, mut atlantic) = (false, false);
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        queue.push_back((row as isize, col as isize));
        const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        while let Some((r, c)) = queue.pop_front() {
            if r < 0 || c < 0 {
                pacific = true;
                continue;
            }
            if r >= heights.len() as isize || c >= heights[0].len() as isize {
                atlantic = true;
                continue;
            }
            
            if pacific && atlantic {
                return true;
            }
            
            if !visited.insert((r, c)) {
                continue;
            }
            let current_height = heights[r as usize][c as usize];
            for &(dr, dc) in &DIRECTIONS {
                let (nr, nc) = (r + dr, c + dc);
                if nr >= 0 && nr < heights.len() as isize 
                    && nc >= 0 && nc < heights[0].len() as isize {
                    let neighbor_height = heights[nr as usize][nc as usize];
                    if current_height >= neighbor_height {
                        queue.push_back((nr, nc));
                    }
                } else {
                    queue.push_back((nr, nc));
                }
            }
        }
        pacific && atlantic
    }
}
