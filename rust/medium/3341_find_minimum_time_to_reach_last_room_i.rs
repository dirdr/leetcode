use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Eq, PartialEq)]
struct State {
    time: i32,
    row: usize,
    col: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());
        let mut heap = BinaryHeap::new();
        heap.push(State {
            time: 0,
            row: 0,
            col: 0,
        });

        let mut best_time = HashMap::new();
        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some(State { time, row, col }) = heap.pop() {
            if let Some(&existing_time) = best_time.get(&(row, col)) {
                if time >= existing_time {
                    continue;
                }
            }
            best_time.insert((row, col), time);

            if row == n - 1 && col == m - 1 {
                return time;
            }

            for &(dr, dc) in &DIRECTIONS {
                let (nr, nc) = (row as isize + dr, col as isize + dc);
                if nr < 0 || nr >= n as isize || nc < 0 || nc >= m as isize {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                let next_time = move_time[nr][nc].max(time) + 1;
                heap.push(State {
                    time: next_time,
                    row: nr,
                    col: nc,
                });
            }
        }
        unreachable!()
    }
}
