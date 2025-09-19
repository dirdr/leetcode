struct Spreadsheet {
    matrix: Vec<Vec<i32>>,
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            matrix: vec![vec![0; 26]; rows as usize],
        }
    }

    fn cell_to_idx(cell: &String) -> (usize, usize) {
        let (col, row) = cell.split_at(1);
        let col = (col.as_bytes()[0] - b'A') as usize;
        let row = row.parse::<usize>().unwrap() - 1;
        (row, col)
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let (r, c) = Self::cell_to_idx(&cell);
        self.matrix[r][c] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    fn get_value(&self, formula: String) -> i32 {
        let expr = formula.trim_start_matches('=');
        let (a, b) = expr.split_once('+').unwrap();
        self.parse_operand(a) + self.parse_operand(b)
    }

    fn parse_operand(&self, s: &str) -> i32 {
        let s = s.trim();
        if let Ok(num) = s.parse::<i32>() {
            num
        } else {
            let (r, c) = Self::cell_to_idx(&s.to_string());
            self.matrix[r][c]
        }
    }
}
