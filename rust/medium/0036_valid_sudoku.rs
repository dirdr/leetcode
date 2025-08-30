impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let (m, n) = (board.len(), board[0].len());
        for i in 0..m {
            let mut mask: u16 = 0;
            for j in 0..n {
                if let Some(digit) = board[i][j].to_digit(10) {
                    let bit = 1 << (digit - 1);
                    if mask & bit != 0 {
                        return false;
                    }
                    mask |= bit;
                }
            }
        }
        for j in 0..n {
            let mut mask: u16 = 0;
            for i in 0..m {
                if let Some(digit) = board[i][j].to_digit(10) {
                    let bit = 1 << (digit - 1);
                    if mask & bit != 0 {
                        return false;
                    }
                    mask |= bit;
                }
            }
        }
        for i in (0..m).step_by(3) {
            for j in (0..n).step_by(3) {
                let mut mask: u16 = 0;
                for di in 0..3 {
                    for dj in 0..3 {
                        if let Some(digit) = board[i + di][j + dj].to_digit(10) {
                            let bit = 1 << (digit - 1);
                            if mask & bit != 0 {
                                return false;
                            }
                            mask |= bit;
                        }
                    }
                }
            }
        }
        true
    }
}
