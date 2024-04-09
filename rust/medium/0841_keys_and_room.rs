use std::collections::HashSet;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited: HashSet<i32> = HashSet::new();
        let mut unlocked: Vec<bool> = vec![false; rooms.len()];
        unlocked[0] = true;
        let mut cloned = rooms.clone();
        Self::dfs(&mut cloned, &mut unlocked, &mut visited, rooms.len(), 0, 0);
        visited.iter().count() == rooms.len()
    }

    pub fn dfs(
        rooms: &mut Vec<Vec<i32>>,
        unlocked: &mut Vec<bool>,
        visited: &mut HashSet<i32>,
        n: usize,
        count: usize,
        current: usize
    ) {
        if (count == n) {
            return;
        }
        if (unlocked[current]) {
            visited.insert(current as i32);
            let keys = rooms[current].clone();
            for j in 0..keys.len() {
                unlocked[keys[j] as usize] = true;
                if !visited.contains(&keys[j]) {
                    Self::dfs(rooms, unlocked, visited, n, count + 1, keys[j] as usize);
                }
            }
        } 
        return;
    }
}
