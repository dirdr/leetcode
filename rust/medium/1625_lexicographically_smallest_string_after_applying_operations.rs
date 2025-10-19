use std::collections::HashSet;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let a = a as usize;
        let b = b as usize;
        let bytes = s.as_bytes();

        let mut cache = HashSet::new();
        
        Self::dfs(bytes, a, b, n, &mut cache);
        
        cache.into_iter()
            .min()
            .map(String::from_utf8)
            .unwrap()
            .unwrap()
    }
    
    fn dfs(current: &[u8], a: usize, b: usize, n: usize, cache: &mut HashSet<Vec<u8>>) {
        if !cache.insert(current.to_vec()) {
            return;
        }
        
        Self::dfs(&Self::add_op(current, a, n), a, b, n, cache);
        Self::dfs(&Self::rot_op(current, b, n), a, b, n, cache);
    }
    
    fn add_op(s: &[u8], a: usize, n: usize) -> Vec<u8> {
        s.iter()
            .enumerate()
            .map(|(i, &digit)| {
                if i % 2 == 1 {
                    let val = (digit - b'0') as usize;
                    b'0' + ((val + a) % 10) as u8
                } else {
                    digit
                }
            })
            .collect()
    }
    
    fn rot_op(s: &[u8], b: usize, n: usize) -> Vec<u8> {
        [&s[n - b..], &s[..n - b]].concat()
    }
}
