use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for i in 0..edges.len() {
            graph[edges[i][0] as usize].push((edges[i][1] as usize, edges[i][2]));
            graph[edges[i][1] as usize].push((edges[i][0] as usize, edges[i][2] * 2));
        }
        let mut dis = vec![i32::MAX; n];
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut queue: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        queue.push((Reverse(0), 0));
        while let Some((Reverse(cost), node)) = queue.pop() {
            visited[node] = true;
            for neighbor in &graph[node] {
                let cost_to_neighbor = cost + neighbor.1;
                let neighbor = neighbor.0;
                if !visited[neighbor] && cost_to_neighbor < dis[neighbor] {
                    dis[neighbor] = cost_to_neighbor;
                    queue.push((Reverse(cost_to_neighbor), neighbor));
                }
            }
        }
        if dis[n - 1] == i32::MAX { -1 } else { dis[n - 1] }
    }
}
