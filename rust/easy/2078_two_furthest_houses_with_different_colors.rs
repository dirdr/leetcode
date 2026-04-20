impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..colors.len() - 1 {
            if colors[i] != colors[colors.len() - 1] {
                max = max.max(colors.len() - 1 - i);
            }
        }
        for i in (1..colors.len()).rev() {
            if colors[i] != colors[0] {
                max = max.max(i);
            }
        }
        max as i32
    }
}
