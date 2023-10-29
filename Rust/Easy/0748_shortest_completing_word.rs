impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut map = std::collections::HashMap::new();
        license_plate.chars().filter(|l| (!l.is_numeric() && *l != ' ')).for_each(|c| {
            map.entry(c.to_lowercase().next().unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
        words.iter().filter(|w| {
            let mut cloned = map.clone();
            for ch in w.chars() {
                if let std::collections::hash_map::Entry::Occupied(mut o) = cloned.entry(ch) {
                    if (*o.get() > 0) {
                        o.insert(*o.get() - 1);
                    }
                }
            }
            cloned.iter().map(|(_, v)| v).sum::<i32>() == 0
        }).min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .to_owned()
    }
}
