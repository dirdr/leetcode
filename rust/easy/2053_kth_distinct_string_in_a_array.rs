use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut freq: HashMap<&str, i32> = HashMap::new();
        for el in arr.iter() {
            freq.entry(el).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut kth = 0;
        for el in arr.iter() {
            if *freq.get(el.as_str()).unwrap() == 1 {
                kth += 1;
            }
            if kth == k {
                return el.to_owned();
            }
        }
        String::from("")
    }
}
