impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let directions = directions.chars().collect::<Vec<_>>();
        let chunks = directions.chunk_by(|a, b| a == b).collect::<Vec<_>>();

        let mut collisions = 0;
        for (i, chunk) in chunks.iter().enumerate() {
            let dir = chunk[0];
            if i == 0 && dir == 'L' || i == chunks.len() - 1 && dir == 'R' {
                continue;
            }
            if dir != 'S' {
                collisions += chunk.len()
            }
        }
        collisions as i32
    }
}
