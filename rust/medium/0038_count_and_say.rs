impl Solution {
    pub fn count_and_say(n: i32) -> String {
        fn agg_freq(number: &[char]) -> Vec<char> {
            let mut count = 1;
            let mut curr = number[0];
            let mut answer = vec![];
            for i in 1..number.len() {
                if number[i] == curr {
                    count += 1;
                    continue;
                }
                answer.extend_from_slice(&[char::from_digit(count as u32, 10).unwrap(), curr]);
                count = 1;
                curr = number[i];
            }
            answer.extend_from_slice(&[char::from_digit(count as u32, 10).unwrap(), curr]);
            answer
        }
        let mut curr = vec!['1'];
        for i in 0..n - 1 {
            curr = agg_freq(&curr);
        }
        curr.into_iter().collect::<String>()
    }
}
