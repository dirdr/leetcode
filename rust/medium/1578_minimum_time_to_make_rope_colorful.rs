impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let zip = colors.chars()
            .zip(needed_time.iter())
            .collect::<Vec<_>>();
        
        let groups = zip.chunk_by(|a, b| a.0 == b.0).collect::<Vec<_>>();
        let mut total = 0;
        for group in groups {
            if group.len() < 2 {
                continue;
            }
            let mut max = 0;
            let mut group_total = 0;
            for el in group {
                group_total += el.1;
                max = max.max(*el.1);
            }
            total += (group_total - max);
        }
        total
    }
}
