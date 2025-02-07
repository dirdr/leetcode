use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut color_count: HashMap<i32, i32> = HashMap::new();
        let mut ball_color: HashMap<i32, i32> = HashMap::new();
        let mut ans = Vec::with_capacity(queries.len());
        let mut distinct_colors = 0;

        for q in queries {
            if let Some(old_color) = ball_color.get(&q[0]) {
                let mut remove = false;
                if let Some(entry) = color_count.get_mut(&old_color) {
                    *entry -= 1;
                    if *entry <= 0 {
                        distinct_colors -= 1;
                        remove = true;
                    }
                }
                if remove {
                    color_count.remove(&old_color);
                }
            }
            ball_color.insert(q[0], q[1]);
            if let Some(color) = color_count.get_mut(&q[1]) {
                *color += 1;
            } else {
                color_count.insert(q[1], 1);
                distinct_colors += 1;
            }
            ans.push(distinct_colors);
        }
        ans
    }
}
