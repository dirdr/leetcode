impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut bs = 0;
        let mut ans = 0;
        for ch in s.bytes() {
            if ch == b'b' {
                bs += 1;
            } else {
                // The answer is the minimum cost to make the string balanced.
                // this can be the once we encounter a a, deleting it, or deleting all the b that we have encountered so far
                ans = (ans + 1).min(bs);
            }
        }
        ans
    }
}
