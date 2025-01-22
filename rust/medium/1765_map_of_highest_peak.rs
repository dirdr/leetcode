use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is_water = is_water;
        let mut queue = VecDeque::new();
        let mut max = 0;
        for i in 0..is_water.len() {
            for j in 0..is_water[0].len() {
                if is_water[i][j] == 1 {
                    queue.push_back((i, j));
                    is_water[i][j] = 0;
                } else {
                    is_water[i][j] = -1;
                }
            }
        }
        let cardinals = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in &cardinals {
                let (nr, nc) = (r as isize + dr, c as isize + dc);
                if nr < 0 || nr >= is_water.len() as isize || nc < 0 || nc >= is_water[0].len() as isize {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if is_water[nr][nc] == -1 {
                    is_water[nr][nc] = is_water[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
        is_water
    }
}
