use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Cell {
    height: i32,
    row: usize,
    col: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut max_height = grid[0][0];
        
        let mut visited = vec![vec![false; n]; n];
        visited[0][0] = true;
        
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(Cell {
            height: grid[0][0],
            row: 0,
            col: 0,
        }));
        
        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        while !visited[n - 1][n - 1] {
            let Reverse(Cell { height, row, col }) = heap.pop().unwrap();
            max_height = max_height.max(height);
            
            for &(dr, dc) in &DIRECTIONS {
                let (r, c) = (row.wrapping_add(dr as usize), col.wrapping_add(dc as usize));
                
                if let Some(&h) = grid.get(r).and_then(|row| row.get(c)) {
                    if !visited[r][c] {
                        visited[r][c] = true;
                        heap.push(Reverse(Cell { height: h, row: r, col: c }));
                    }
                }
            }
        }
        
        max_height.max(grid[n - 1][n - 1])
    }
}
