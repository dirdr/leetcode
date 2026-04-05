impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut x, mut y) = (0, 0);
        for ch in moves.chars() {
            match ch {
                'R' => x += 1,
                'D' => y += 1,
                'L' => x -= 1,
                'U' => y -= 1,
                _ => unreachable!()
            }
        }
        x == 0 && y == 0
    }
}
