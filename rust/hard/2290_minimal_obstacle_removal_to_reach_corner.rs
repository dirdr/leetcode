use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dist = vec![vec![i32::MAX; n]; m];
        let mut deque = VecDeque::new();
        let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        deque.push_front((0, 0, 0));
        dist[0][0] = 0;
        while let Some((r, c, obstacles)) = deque.pop_front() {
            if r == m - 1 && c == n - 1 {
                return obstacles;
            }

            for (dr, dc) in dirs.iter() {
                let next_r = r as isize + dr;
                let next_c = c as isize + dc;

                if next_r < 0 || next_r >= m as isize || 
                   next_c < 0 || next_c >= n as isize {
                    continue;
                }

                let (next_r, next_c) = (next_r as usize, next_c as usize);
                let next_obstacles = obstacles + grid[next_r][next_c];
                if next_obstacles < dist[next_r][next_c] {
                    dist[next_r][next_c] = next_obstacles;
                    if grid[next_r][next_c] == 0 {
                        deque.push_front((next_r, next_c, next_obstacles));
                    } else {
                        deque.push_back((next_r, next_c, next_obstacles));
                    }
                }
            }
        }
        -1
    }
}
