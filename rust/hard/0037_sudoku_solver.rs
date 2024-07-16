impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut availables = vec![vec![vec![true; 9]; 9]; 9];
        Self::update_availables(&board, &mut availables);
        Self::backtrack(board, &mut availables);
    }

    fn backtrack(board: &mut Vec<Vec<char>>, availables: &mut Vec<Vec<Vec<bool>>>) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    for num in 1..=9 {
                        if availables[r][c][num - 1] {
                            let char_num = std::char::from_digit(num as u32, 10).unwrap();
                            let prev_availables = availables.clone();
                            board[r][c] = char_num;
                            Self::update_availables(board, availables);
                            if Self::backtrack(board, availables) {
                                return true;
                            }
                            board[r][c] = '.';
                            *availables = prev_availables;
                        }
                    }
                    return false;                 
                }
            }
        }
        true
    }

    fn update_availables(board: &Vec<Vec<char>>, availables: &mut Vec<Vec<Vec<bool>>>) {
        for r in 0..9 {
            for c in 0..9 {
                if let Some(num) = board[r][c].to_digit(10) {
                    for dc in 0..9 {
                        availables[r][dc][num as usize - 1] = false;
                    }
                    for dr in 0..9 {
                        availables[dr][c][num as usize - 1] = false;
                    }
                    let (box_start_r, box_start_c) = (r / 3 * 3, c / 3 * 3);
                    for dr in 0..3 {
                        for dc in 0..3 {
                            availables[box_start_r + dr][box_start_c + dc][num as usize - 1] = false;
                        }
                    }
                }
            }
        }
    }
}
