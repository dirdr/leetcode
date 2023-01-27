impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        pub fn dfs(graph: &Vec<Vec<i32>>, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, node: i32) {
            // base case
            if node as usize == graph.len() - 1 {
                res.push(path.to_vec());
                return;
            }
            // dfs into neighbour
            for nxt in graph[node as usize].iter() {
                path.push(*nxt);
                dfs(graph, res, path, *nxt);
                path.pop();
            }
        }
        let mut res = Vec::new();
        let mut path = vec![0];
        dfs(&graph, &mut res, &mut path, 0);
        res
    }
}
