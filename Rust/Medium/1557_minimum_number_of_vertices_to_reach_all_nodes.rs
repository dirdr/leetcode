use std::collections::HashSet;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited: HashSet<i32> = HashSet::new();
        for i in 0..edges.len() {
            visited.insert(edges[i][1]);
        }
        (0..n).into_iter()
            .map(|x| x as i32)
            .filter(|x| !visited.contains(&x))
            .collect()
    }
}
