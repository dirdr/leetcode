impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n as usize]; n as usize];
        let mut answer = vec![];
        dfs(&mut board, &mut answer, &mut vec![vec![true; n as usize]; n as usize], 0);
        fn dfs(board: &mut Vec<Vec<char>>, answer: &mut Vec<Vec<String>>, availables: &mut Vec<Vec<bool>>, curr_row: usize) {
            let n = board.len();
            if curr_row == n {
                answer.push(board.clone()
                    .iter()
                    .map(|e| e.iter().collect::<String>())
                    .collect::<Vec<_>>()
                );
                return;
            }
            for j in 0..n {
                if availables[curr_row][j] {
                    board[curr_row][j] = 'Q';
                    let prev = availables.clone();
                    Solution::update_availables(availables, (curr_row, j));

                    dfs(board, answer, availables, curr_row + 1);

                    board[curr_row][j] = '.';
                    *availables = prev;
                }
            }

        }
        answer
    }

    fn update_availables(board: &mut Vec<Vec<bool>>, pos: (usize, usize)) {
        let n = board.len();
        let (row, col) = pos;
        for i in 0..n {
            board[i][col] = false;
            board[row][i] = false;
        }
        let directions: [(isize, isize); 4] = [(-1, -1), (1, 1), (-1, 1), (1, -1)];
        for (di, dj) in directions.iter() {
            let mut i = row as isize + di;
            let mut j = col as isize + dj;
            while i >= 0 && i < n as isize && j >= 0 && j < n as isize {
                board[i as usize][j as usize] = false;
                i += di;
                j += dj;
            }
        }
    }
}
