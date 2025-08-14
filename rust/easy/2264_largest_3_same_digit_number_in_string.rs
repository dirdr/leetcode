impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
        let (mut curr, mut streak, mut max) = (num[0], 1, -1);
        for i in 1..num.len() {
            if num[i] == curr {
                streak += 1;
            } else {
                curr = num[i];
                streak = 1;
            }
            if streak == 3 {
                max = max.max(num[i]);
                streak = 1;
            }
        }
        if max == -1 {
            return "".to_string();
        }
        char::from_digit(max as u32, 10).unwrap().to_string().repeat(3)
    }
}
