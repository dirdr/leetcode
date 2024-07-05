impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut answer = vec![];
        fn dfs(s: &[char], partition: &mut Vec<String>, answer: &mut Vec<Vec<String>>, pos: usize) {
            if partition.iter().map(|s| s.len()).sum::<usize>() == s.len() {
                answer.push(partition.clone());
                return;
            }
            for i in 1..(s.len() - pos + 1) {
                let substr = &s[pos..(pos + i)];
                if substr.iter().eq(substr.iter().rev()) {
                    partition.push(substr.iter().cloned().collect::<String>());
                    dfs(s, partition, answer, pos + i);
                    partition.pop();
                }
                
            }
        }
        dfs(&s.chars().collect::<Vec<char>>(), &mut vec![], &mut answer, 0);
        answer
    }
}
