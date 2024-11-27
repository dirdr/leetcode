use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adjacency_list = (0..n - 1).map(|i| vec![i as usize + 1]).collect::<Vec<_>>();
        adjacency_list.push(vec![usize::MAX]);
        let mut answer = vec![];
        for q in &queries {
            adjacency_list[q[0] as usize].push(q[1] as usize);
            answer.push(Self::get_shortest_path(&adjacency_list, n));
        }
        answer
    }

    pub fn get_shortest_path(adjacency_list: &Vec<Vec<usize>>, n: usize) -> i32 {
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;
        while !queue.is_empty() {
            let front = queue.pop_front().unwrap();
            for &nei in &adjacency_list[front] {
                if nei != usize::MAX && dist[nei] == i32::MAX {
                    queue.push_back(nei);
                    dist[nei] = dist[front] + 1;
                }
                if nei == n - 1 {
                    return dist[n - 1];
                }
            }
        }
        -1
    }
}
