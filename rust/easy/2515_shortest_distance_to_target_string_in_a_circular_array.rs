impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let idx = start_index as usize;
        let n = words.len();
        let mut min = i32::MAX;
        for i in 0..n {
            if words[(idx + i) % n] == target {
                min = min.min(i as i32);
            }
        }
        for i in 0..n {
            if words[(idx - i + n) % n] == target {
                min = min.min(i as i32);
            }
        }
        if min == i32::MAX {
            return -1
        }
        min
    }
}
