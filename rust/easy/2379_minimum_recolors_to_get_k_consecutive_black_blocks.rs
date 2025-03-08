impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut w = 0;
        let mut min = i32::MAX;
        let blocks = blocks.chars().collect::<Vec<_>>();
        while r < blocks.len() {
            if blocks[r] == 'W' {
                w += 1;
            }
            if r - l + 1 >= k as usize {
                min = min.min(w);
                if blocks[l] == 'W' {
                    w -= 1;
                }
                l += 1;
            }
            r += 1;
        }
        min
    }
}
