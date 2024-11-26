use std::collections::HashSet;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut ins: HashSet<i32> = HashSet::from_iter((0..edges.len()).map(|i| edges[i][1]));
        let champions = (0..n).filter(|i| !ins.contains(&i)).collect::<Vec<_>>();
        if champions.len() > 1 {
            return -1;
        }
        champions[0]
    }
}
