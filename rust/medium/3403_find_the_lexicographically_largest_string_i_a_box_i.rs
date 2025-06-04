impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends < 2 {
            return word;
        }
        let mut largest: &[char] = &[];
        let win = word.len() - num_friends as usize + 1;
        let word = word.chars().collect::<Vec<_>>();
        let mut l = 0;
        for r in 0..word.len() {
            while r - l + 1 > win {
                l += 1;
            }
            while r == word.len() - 1 && r - l > 0 {
                if &word[l..=r] > largest {
                    largest = &word[l..=r];
                }
                l += 1;
            }
            if &word[l..=r] > largest {
                largest = &word[l..=r];
            }
        }
        largest.iter().collect::<String>()
    }
}
