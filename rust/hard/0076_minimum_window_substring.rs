impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return String::new();
        }
        let mut found = false;
        let mut map = std::collections::HashMap::new();
        let mut current_min: Vec<char> = s.chars().collect();
        let (mut l, mut r) = (0usize, 0usize);
        for c in t.chars() {
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        let s: Vec<char> = s.chars().collect();
        while r < s.len() {
            if let Some(right_count) = map.get_mut(&s[r]) {
                *right_count -= 1;
            }
            while map.get(&s[l]).is_none()
                || (map.get(&s[l]).is_some() && map.get(&s[l]).unwrap().to_owned() < 0)
            {
                map.entry(s[l]).and_modify(|e| *e += 1);
                l += 1;
                if l == s.len() {
                    return String::new();
                }
            }
            if map.values().all(|&v| v <= 0) {
                found = true;
                //println!("left : {}, right : {}", l, r);
                let new_min = match current_min.len().cmp(&(r - l + 1)) {
                    std::cmp::Ordering::Greater => s[l..r + 1].to_owned(),
                    _ => current_min,
                };
                current_min = new_min;
            }
            r += 1;
        }
        if !found {
            return String::new("");
        }
        current_min.iter().collect::<String>()
    }
}
