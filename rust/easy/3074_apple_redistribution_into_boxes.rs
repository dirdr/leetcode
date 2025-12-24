impl Solution {
    pub fn minimum_boxes(mut apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_unstable_by(|a, b| b.cmp(&a));
        let (mut i, mut j) = (0, 0);
        while i < apple.len() {
            if apple[i] > capacity[j] {
                apple[i] -= capacity[j];
                j += 1;
            } else {
                capacity[j] -= apple[i];
                i += 1;
            }
        }
        j as i32 + 1
    }
}
