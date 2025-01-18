use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
struct Node {
    cost: i32,
    row: usize,
    col: usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let directions = [(0, 1, 1), (0, -1, 2), (1, 0, 3), (-1, 0, 4)];
        let mut queue = BinaryHeap::new();
        let m = grid.len();
        let n = grid[0].len();
        let mut cost_map: HashMap<(usize, usize), i32> = HashMap::new();

        cost_map.insert((0, 0), 0);
        queue.push(Node {
            cost: 0,
            row: 0,
            col: 0
        });
        
        while let Some(Node {cost, row, col}) = queue.pop() {
            if row == m - 1 && col == n - 1 {
                return cost;
            }
            for &(dr, dc, dir_num) in &directions {
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                let penalty = if grid[row][col] == dir_num { 0 } else { 1 };
                let new_cost = cost + penalty;
                if let Some(&existing_cost) = cost_map.get(&(nr, nc)) {
                    if new_cost < existing_cost {
                        cost_map.insert((nr, nc), new_cost);
                        queue.push(Node {
                            cost: new_cost,
                            row: nr,
                            col: nc,
                        });
                    }
                } else {
                    cost_map.insert((nr, nc), new_cost);
                    queue.push(Node {
                        cost: new_cost,
                        row: nr,
                        col: nc,
                    });
                }
            }
        }
        -1
    }
}
