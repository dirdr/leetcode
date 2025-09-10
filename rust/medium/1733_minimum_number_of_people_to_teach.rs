use std::collections::HashSet;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let m = languages.len();
        let mut langs: Vec<HashSet<i32>> = languages.into_iter()
            .map(|v| v.into_iter().collect())
            .collect();

        let mut problematic = Vec::new();
        for f in friendships {
            let (u, v) = (f[0] as usize - 1, f[1] as usize - 1);
            if langs[u].intersection(&langs[v]).next().is_none() {
                problematic.push((u, v));
            }
        }

        if problematic.is_empty() {
            return 0;
        }

        let mut result = i32::MAX;

        for lang in 1..=n {
            let mut to_teach: HashSet<usize> = HashSet::new();
            for &(u, v) in &problematic {
                if !langs[u].contains(&lang) {
                    to_teach.insert(u);
                }
                if !langs[v].contains(&lang) {
                    to_teach.insert(v);
                }
            }
            result = result.min(to_teach.len() as i32);
        }

        result
    }
}
