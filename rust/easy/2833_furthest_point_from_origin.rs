impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let moves = moves.chars().collect::<Vec<char>>();
        let fl = moves.iter().map(|&c| if c == '_' { 'L' } else { c }).collect::<Vec<char>>();
        let rl = moves.iter().map(|&c| if c == '_' { 'R' } else { c }).collect::<Vec<char>>();
        let mut distance: i32 = 0;
        let mut max = 0;
        for i in 0..fl.len() {
            if fl[i] == 'L' {
                distance -= 1;
            } else {
                distance += 1;
            }
        }
        max = max.max(distance.abs());
        distance = 0;
        for i in 0..rl.len() {
            if rl[i] == 'R' {
                distance += 1;
            } else {
                distance -= 1;
            }
        }
        max = max.max(distance.abs());
        max
    }
}
