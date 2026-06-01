impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable_by(|a, b| b.cmp(&a));
        cost.chunks(3).fold(0, |acc, g| {
            match g.len() {
                1 => acc + g[0],
                2 | 3 => {
                    acc + g[0] + g[1]
                },
                _ => unreachable!()
            }
        })
    }
}
