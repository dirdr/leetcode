use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in board.clone() {
            if Solution::check_if_duplicate(&row) {return false}
        }
        let mut transposed: Vec<Vec<char>> = vec![Vec::with_capacity(9); 9];
        for row in board.clone() {
            for i in 0..9 {
                transposed[i].push(row[i]);
            } 
        }
        for row in transposed {
            if Solution::check_if_duplicate(&row) {return false}
        }
        let mut row_offset = 0;
        let mut col_offset = 0;
        for _ in 0..3 {
            for _ in 0..3 {
                let mut temp: Vec<char> = Vec::new();
                for i in 0..3 {
                    for j in 0..3 {
                        temp.push(board[i + row_offset][j + col_offset]);
                    }
                }
                if Solution::check_if_duplicate(&temp) {return false}
                col_offset += 3;
            }
            col_offset = 0;
            row_offset += 3;
        }
        true
    }

    pub fn check_if_duplicate(vec: &Vec<char>) -> bool { 
        let mut set: HashSet<char> = HashSet::new();
        !vec.into_iter()
            .filter(|e| **e != '.')
            .all(|e| set.insert(*e))
    }
}
