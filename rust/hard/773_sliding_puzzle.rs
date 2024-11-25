use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut queue: VecDeque<Vec<Vec<i32>>> = VecDeque::new();
        let mut visited: HashSet<Vec<i32>> = HashSet::new();
        let mut count = 0;

        let initial_state = board.clone();
        queue.push_back(initial_state);
        visited.insert(board.into_iter().flatten().collect());

        while !queue.is_empty() {
            let mut next: VecDeque<Vec<Vec<i32>>> = VecDeque::new();
            while let Some(state) = queue.pop_front() {
                if Self::is_solved(&state) {
                    return count;
                }
                let states = Self::generate_next_state(&state);
                for new_state in states {
                    let flat_new_state = new_state.iter().flatten().cloned().collect::<Vec<i32>>();
                    if visited.insert(flat_new_state.clone()) {
                        next.push_back(new_state);
                    }
                }
            }
            count += 1;
            std::mem::swap(&mut queue, &mut next);
        }
        -1
    }

    pub fn generate_next_state(board: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut empty_pos = (2, 3);
        for r in 0..2 {
            for c in 0..3 {
                if board[r][c] == 0 {
                    empty_pos = (r, c);
                    break;
                }
            }
            if empty_pos != (2, 3) {
                break;
            }
        }
        let mut next_states = Vec::new();
        for &(dx, dy) in &DIRS {
            let new_x = (empty_pos.0 as isize + dx) as usize;
            let new_y = (empty_pos.1 as isize + dy) as usize;
            if new_x < 2 && new_y < 3 {
                let mut new_board = board.clone();
                new_board[empty_pos.0][empty_pos.1] = new_board[new_x][new_y];
                new_board[new_x][new_y] = 0;
                next_states.push(new_board);
            }
        }
        next_states
    }

    pub fn is_solved(board: &Vec<Vec<i32>>) -> bool {
        let target = [1, 2, 3, 4, 5, 0];
        let mut flat = board.iter().flatten().copied().collect::<Vec<_>>();
        flat == target
    }
}
