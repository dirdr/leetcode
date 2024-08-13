impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut str = x.to_string().chars().collect::<Vec<_>>();
        let (mut l, mut r) = (0, str.len() - 1);
        if str[l] == '-' {
            l += 1;
        }
        while str[r] == '0' {
            r -= 1;
        }
        let end = r;
        while l < r {
            (str[l], str[r]) = (str[r], str[l]);
            l += 1;
            r -= 1;
        }
        match str[..=end].iter().collect::<String>().parse::<i32>() {
            Ok(r) => r,
            Err(e) => 0,
        }
    }
}
