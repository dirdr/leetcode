impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        fn longest_subsequences_starting_with(words: &Vec<String>, groups: &Vec<i32>, starting: i32) -> Vec<String> {
            let mut buffer = vec![];
            let mut starting = starting;
            for i in 0..groups.len() {
                if groups[i] == starting {
                    starting = 1 - starting;
                    buffer.push(words[i].clone());
                }
            }
            buffer
        }

        let starting_zero = longest_subsequences_starting_with(&words, &groups, 0);
        let starting_one = longest_subsequences_starting_with(&words, &groups, 1);

        if starting_zero.len() > starting_one.len() {
            starting_zero
        } else {
            starting_one
        }
    }
}
