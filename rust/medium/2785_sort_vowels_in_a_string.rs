impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut vowels_in_s = vec![];
        for &ch in &s {
            if VOWELS.contains(&ch) {
                vowels_in_s.push(ch);
            }
        }
        vowels_in_s.sort();
        let mut t = vec![' '; s.len()];
        let mut curr = 0;

        for i in 0..s.len() {
            match VOWELS.contains(&s[i]) {
                true => {
                    t[i] = vowels_in_s[curr];
                    curr += 1;
                },
                false => {
                    t[i] = s[i];
                }
            }
        }
        t.into_iter().collect::<String>()
    }
}
