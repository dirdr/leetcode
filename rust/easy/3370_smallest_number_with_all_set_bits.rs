impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut curr = 0;
        while 2i32.pow(curr) <= n {
            curr += 1;
        }
        2i32.pow(curr) - 1
    }
}
