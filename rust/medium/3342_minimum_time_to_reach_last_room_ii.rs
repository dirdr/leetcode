use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Eq, PartialEq)]
struct State {
    time: i32,
    row: usize,
    col: usize,
    parity: u8,
}


impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());

        let mut best = vec![vec![[i32::MAX; 2]; m]; n];
        best[0][0][0] = 0;

        let mut pq = BinaryHeap::with_capacity(2 * n * m);
        pq.push(State { time: 0, row: 0, col: 0, parity: 0 });

        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some(State { time, row, col, parity }) = pq.pop() {
            if time != best[row][col][parity as usize] {
                continue;
            }

            if row == n - 1 && col == m - 1 {
                return time;
            }

            for (dr, dc) in DIRS {
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if nr < 0 || nr >= n as isize || nc < 0 || nc >= m as isize {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);

                let depart = time.max(move_time[nr][nc]);
                let move_cost = if parity == 0 { 1 } else { 2 };
                let next_time = depart + move_cost;
                let next_parity = parity ^ 1;

                if next_time < best[nr][nc][next_parity as usize] {
                    best[nr][nc][next_parity as usize] = next_time;
                    pq.push(State {
                        time: next_time,
                        row: nr,
                        col: nc,
                        parity: next_parity,
                    });
                }
            }
        }
        unreachable!()
    }
}
