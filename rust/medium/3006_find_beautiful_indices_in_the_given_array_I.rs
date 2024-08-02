impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        let mut search_start = 0;
        let mut b_idx = vec![];
        let mut offset = 0;
        while search_start <= (s.len() - b.len()) {
            if let Some(si) = &s[search_start..].find(&b[..]) {
                let shifted = *si + offset;
                b_idx.push(shifted);
                search_start = shifted + 1;
                offset += si + 1;
            } else {
                break;
            }
        }
        let mut search_start = 0;
        let mut answer = vec![];
        let mut offset = 0;
        while search_start <= (s.len() - a.len()) {
            if let Some(si) = &s[search_start..].find(&a[..]) {
                let shifted = *si + offset;
                if b_idx.iter().any(|&i| (i as i32 - (*si as i32 + offset as i32)).abs() <= k) {
                    answer.push(shifted as i32);
                }
                search_start = shifted + 1;
                offset += si + 1;
            } else {
                break;
            }
        }
        answer
    }
}
