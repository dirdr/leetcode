impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let start = start.chars().collect::<Vec<_>>();
        let target = target.chars().collect::<Vec<_>>();
        let start_pieces: Vec<char> = start.iter().filter(|&&c| c != '_').cloned().collect();
        let target_pieces: Vec<char> = target.iter().filter(|&&c| c != '_').cloned().collect();
        if start_pieces != target_pieces {
            return false;
        }
        let (mut i, mut j) = (0, 0);
        while i < start.len() && j < target.len() {
            while i < start.len() && start[i] == '_' {
                i += 1;
            }
            while j < target.len() && target[j] == '_' {
                j += 1;
            }
            if i == start.len() || j == target.len() {
                break;
            }
            if start[i] == 'L' && i < j {
                return false;
            }
            if start[i] == 'R' && i > j {
                return false;
            }
            i += 1;
            j += 1;
        }
        true
    }
}
