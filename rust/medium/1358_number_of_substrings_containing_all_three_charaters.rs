impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let (mut a, mut b, mut c) = (0, 0, 0);
        let (mut l, mut r) = (0, 0);
        let mut result = 0;
        while r < s.len() {
            match s[r] {
                'a' => a += 1,
                'b' => b += 1,
                'c' => c += 1,
                _ => unreachable!(),
            }
            while a >= 1 && b >= 1 && c >= 1 {
                result += s.len() - r;
                match s[l] {
                    'a' => a -= 1,
                    'b' => b -= 1,
                    'c' => c -= 1,
                    _ => unreachable!(),
                }
                l += 1;
            }
            r += 1;
        }
        result as i32
    }
}
