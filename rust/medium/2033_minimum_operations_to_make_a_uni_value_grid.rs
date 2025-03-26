impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut flattened = grid.into_iter().flatten().collect::<Vec<_>>();
        flattened.sort();
        let median = Self::median(&flattened);
        let reference = flattened[0] % x;
        let mut steps = 0;
        for &el in &flattened {
            if el % x != reference {
                return -1;
            }
            let diff = (median / x) - (el / x);
            steps += diff.abs();
        }
        steps
    }

    pub fn median(array: &[i32]) -> i32 {
        let n = array.len();
        let mid = n / 2;
        array[mid]
    }
}
