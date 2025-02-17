impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        fn backtrack(current: &mut Vec<char>, tiles: &Vec<char>, used: &mut Vec<bool>) -> i32 {
            let mut count = if !current.is_empty() { 1 } else { 0 };
            let mut used_chars = std::collections::HashSet::new();
            
            for i in 0..tiles.len() {
                if used[i] || used_chars.contains(&tiles[i]) {
                    continue;
                }
                
                used[i] = true;
                current.push(tiles[i]);
                used_chars.insert(tiles[i]);
                
                count += backtrack(current, tiles, used);
                
                used[i] = false;
                current.pop();
            }
            
            count
        }
        
        backtrack(
            &mut Vec::new(),
            &tiles.chars().collect(),
            &mut vec![false; tiles.len()]
        )
    }
}
