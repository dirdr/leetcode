use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut queue: VecDeque<char> = VecDeque::new();
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut max = 0;
        for c in s.chars() {
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            let max_char = map
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _)| k)
                .unwrap()
                .to_owned();
            let mut diff: usize = map.iter()
                .filter(|&(&k, _)| k != max_char)
                .map(|(_, v)| v)
                .sum();
            while diff as i32 > k {
                let removed = queue.pop_front().unwrap();
                map.entry(removed).and_modify(|e| *e -= 1);
                if let Some(&mut value) = map.get_mut(&removed) {
                    if value < 0 {
                        map.remove(&removed);
                    }
                }
                diff -= 1;
            }
            queue.push_back(c);
            max = std::cmp::max(max, queue.len() as i32);
        }
        max
    }
}
