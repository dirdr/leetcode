impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions.iter()
            .map(|rec| (rec[0] * rec[0] + rec[1] * rec[1], rec[0] * rec[1]))
            .max()
            .unwrap().1
    }
}
