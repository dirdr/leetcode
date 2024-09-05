impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let (n, mean) = (n as usize, mean as usize);
        let m = rolls.len();
        let fill = (n + m) * mean - rolls.iter().sum::<i32>() as usize;
        if fill > 6 * n || fill < n {
            return vec![];
        }
        let (q, r) = (fill / n, fill % n);
        let mut answer = vec![0; n];
        for i in 0..n {
            answer[i] = q as i32;
        }
        for i in 0..r {
            answer[i] += 1; 
        }
        answer
    }
}
