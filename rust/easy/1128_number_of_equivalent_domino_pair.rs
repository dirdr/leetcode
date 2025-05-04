use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut freqs = HashMap::new();
        let mut count = 0;
        for d in &dominoes {
            let (a, b) = if d[0] > d[1] { (d[1], d[0]) } else { (d[0], d[1]) };
            *freqs.entry((a, b)).or_insert(0) += 1;
        }
        for freq in freqs.values() {
            count += ((freq - 1) * freq) / 2;
        }
        count
    }
}
