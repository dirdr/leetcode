use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let lens = (1..=s.len() as i32 - 2).collect::<Vec<_>>();
        if !Self::is_possible(&s, 1) {
            return -1;
        }
        let idx = lens.partition_point(|&len| Self::is_possible(&s, len));
        lens[idx - 1]
    }

    pub fn is_possible(s: &String, len: i32) -> bool {
        let chars = s.chars().collect::<Vec<_>>();
        let mut map: HashMap<&[char], i32> = HashMap::new();
        for window in chars.windows(len as usize) {
            if window.iter().all(|&c| c == window[0]) {
                map.entry(window).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        map.values().any(|&v| v >= 3)
    }
}
