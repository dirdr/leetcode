impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut answer = vec![];
        if n % 2 != 0 {
            answer.push(0);
        }
        let mut current = 1;
        while answer.len() < n  {
            answer.push(current);
            answer.push(current * -1);
            current += 1;
        }
        answer
    }
}
