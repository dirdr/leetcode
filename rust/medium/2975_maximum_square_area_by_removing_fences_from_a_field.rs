use std::collections::HashSet;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, mut h_fences: Vec<i32>, mut v_fences: Vec<i32>) -> i32 {
        h_fences.sort_unstable();
        v_fences.sort_unstable();
        h_fences.insert(0, 1);
        v_fences.insert(0, 1);
        h_fences.push(m);
        v_fences.push(n);
        let mut h_edges = HashSet::new();
        let mut v_edges = HashSet::new();
        for i in 0..h_fences.len() - 1 {
            for j in i + 1..h_fences.len() {
                h_edges.insert(h_fences[j] - h_fences[i]);
            }
        }
        for i in 0..v_fences.len() - 1 {
            for j in i + 1..v_fences.len() {
                v_edges.insert(v_fences[j] - v_fences[i]);
            }
        }
        let mut max = 0;
        const MODULO: i128 = 1_000_000_007;
        let mut found = false;
        for &entry in h_edges.iter() {
            if v_edges.contains(&entry) {
                found = true;
                max = max.max(entry as i128 * entry as i128);
            }
        }
        if found { (max % MODULO) as i32 } else { -1 }
    }
}
