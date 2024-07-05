impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board[0].is_empty() || word.is_empty() {
            return false;
        } 
        let m = board.len();
        let n = board[0].len();
        let mut board = board;
        let chars: Vec<char> = word.chars().collect(); 
        for i in 0..m {
            for j in 0..n {
                if Self::backtrack(&mut board, &chars, i, j, 0) {
                    return true;
                }
            }
        } 
        false
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        word: &[char],
        i: usize,
        j: usize,
        word_idx: usize,
    ) -> bool {
        if word_idx == word.len() {
            return true;
        }
        if i >= board.len() || j >= board[0].len() || board[i][j] != word[word_idx] {
            return false;
        }
        
        let temp = board[i][j];
        board[i][j] = '#';
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        let mut found = false;
        for (or, oc) in directions {
            let (ni, nj) = (i as isize + or, j as isize + oc);
            if ni >= 0 && ni < board.len() as isize && nj >= 0 && nj < board[0].len() as isize {
                found = Self::backtrack(board, word, ni as usize, nj as usize, word_idx + 1);
                if found {
                    break;
                }
            }
        } 
        board[i][j] = temp;
        if !found && word_idx == word.len() - 1 {
            return true;
        }
        found
    }
}
