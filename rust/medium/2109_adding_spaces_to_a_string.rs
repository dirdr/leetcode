impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut s = s;
        let mut offset = 0;
        for space in spaces {
            s.insert(space as usize + offset, ' ');
            offset += 1;
        }
        s
    }
}
