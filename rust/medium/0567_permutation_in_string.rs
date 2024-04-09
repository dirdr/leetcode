impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = std::collections::HashMap::new();
        for c in s1.chars() {
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        let (mut l, mut r) = (0 as usize, 0 as usize);
        let s2 = s2.chars().collect::<Vec<char>>();
        while r < s2.len() {
            if let Some(count) = map.get_mut(&s2[r]) {
                *count -= 1;
            }
            if r - l + 1 == s1.len() {
                if map.iter().all(|(_, &v)| v == 0) {
                    return true;
                }
                if let Some(count) = map.get_mut(&s2[l]) {
                    *count += 1;
                }
                l += 1;
            }
            r += 1;
        }
        false
    }
}
