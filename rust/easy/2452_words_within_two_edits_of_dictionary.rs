impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut answer = vec![];
        for w in queries {
            for ow in &dictionary {
                let mut diff = 0;
                let wc = w.chars().collect::<Vec<_>>();
                let owc = ow.chars().collect::<Vec<_>>();
                for i in 0..w.len() {
                    if wc[i] != owc[i] {
                        diff += 1;
                    }
                }
                if diff <= 2 {
                    answer.push(w.clone());
                    break;
                }
            }
        }
        answer
    }
}
