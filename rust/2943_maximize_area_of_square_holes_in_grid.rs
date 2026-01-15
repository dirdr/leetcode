impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, mut h_bars: Vec<i32>, mut v_bars: Vec<i32>) -> i32 {
        h_bars.sort_unstable();
        v_bars.sort_unstable();
        fn find_max_consecutive(arr: &[i32]) -> i32 {
            let mut max = 1;
            let mut curr = 1;
            for i in 1..arr.len() {
                if arr[i] == arr[i - 1] + 1 {
                    curr += 1;
                    max = max.max(curr);
                } else {
                    max = max.max(curr);
                    curr = 1;
                }
            }
            max
        }
        let min = std::cmp::min(find_max_consecutive(&h_bars) + 1, find_max_consecutive(&v_bars) + 1);
        min * min
    }
}
