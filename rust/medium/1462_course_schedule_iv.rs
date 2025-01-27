use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut adj = vec![vec![]; num_courses as usize];
        for p in &prerequisites {
            adj[p[0] as usize].push(p[1] as usize);
        }
        let mut reachable: HashMap<usize, HashSet<usize>> = HashMap::new();
        fn dfs(
            course: usize,
            adj: &[Vec<usize>],
            reachable: &mut HashMap<usize, HashSet<usize>>
        ) -> HashSet<usize> {
            if let Some(cached) = reachable.get(&course) {
                return cached.clone();
            }
            let mut set = HashSet::new();
            for &next in &adj[course] {
                set.insert(next);
                set.extend(dfs(next, adj, reachable));
            }
            reachable.insert(course, set.clone());
            set
        }
        for course in 0..num_courses as usize {
            if !reachable.contains_key(&course) {
                dfs(course, &adj, &mut reachable);
            }
        }
        queries
            .iter()
            .map(|q| {
                reachable
                    .get(&(q[0] as usize))
                    .map_or(false, |set| set.contains(&(q[1] as usize)))
            })
            .collect()
    }
}
