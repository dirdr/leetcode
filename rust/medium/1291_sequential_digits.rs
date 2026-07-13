impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        fn generate(num: i32, c: i32, low: i32, high: i32, buff: &mut Vec<i32>) {
            if num > high {
                return;
            }
            if num >= low {
                buff.push(num);
            }
            if c > 9 {
                return;
            }
            generate(num * 10 + c, c + 1, low, high, buff);
        }
        let mut answer = vec![];
        for i in 1..= 8 {
            let mut temp = vec![];
            generate(i, i + 1, low, high, &mut temp);
            answer.extend(temp);
        }
        answer.sort_unstable();
        answer
    }
}
