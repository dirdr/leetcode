impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        fn backtrack(n: i32, current: &mut Vec<i32>, used: &mut Vec<bool>) -> i32 {
            if current.len() == n as usize {
                return 1;
            }
            let mut count = 0;
            for i in 1..=(n as usize) {
                if !used[i - 1] && ((current.len() + 1) % i == 0 || i % (current.len() + 1) == 0) {
                    used[i - 1] = true;
                    current.push(i as i32);
                    count += backtrack(n, current, used);
                    current.pop();
                    used[i - 1] = false;
                }
            }
            count
        }
        backtrack(n, &mut vec![], &mut vec![false; n as usize])
    }
}
