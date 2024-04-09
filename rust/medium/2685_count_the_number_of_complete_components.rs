use std::collections::HashSet;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 0..edges.len() as usize {
            graph[edges[i][0] as usize].push(edges[i][1] as usize);
            graph[edges[i][1] as usize].push(edges[i][0] as usize);
        }
        // count the number of node and the number of edges
        // considering a non directional edges count as two edges,
        // one between a and b and one between b and a
        let mut visited = HashSet::new();
        let mut answer = 0;
        for i in 0..n {
            if !visited.contains(&i) {
                let mut edgeCount = 0;
                let mut nodeCount = 0;
                Self::dfs(&mut graph, &mut visited, i, &mut edgeCount, &mut nodeCount);
                if nodeCount * (nodeCount - 1) == edgeCount { answer += 1 }
            }
        }
        answer
    }

    pub fn dfs(
        graph: &mut Vec<Vec<usize>>,
        visited: &mut HashSet<usize>,
        current: usize,
        edgeCount: &mut usize,
        nodeCount: &mut usize
    )   {
        if visited.contains(&current) { return }
        visited.insert(current);
        *nodeCount += 1;
        *edgeCount += graph[current].len();

        for u in graph[current].clone() {
            Self::dfs(graph, visited, u, edgeCount, nodeCount);
        }
    }
}
