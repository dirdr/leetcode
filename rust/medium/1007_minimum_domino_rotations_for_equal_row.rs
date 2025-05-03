impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let (mut t, mut b, mut s) = (vec![0; 7], vec![0; 7], vec![0; 7]);
        let n = tops.len();
        for i in 0..n {
            t[tops[i] as usize] += 1;
            b[bottoms[i] as usize] += 1;
            s[tops[i] as usize] += (tops[i] == bottoms[i]) as usize;
        }
        let mut min = usize::MAX;
        for i in 1..=6 {
            if t[i] + b[i] - s[i] == n {
                min = min.min(n - t[i]).min(n - b[i]);
            }
        }
        min as i32
    }
}
