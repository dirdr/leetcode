use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut map = HashMap::new();
        favorite_companies.iter().enumerate().for_each(|(i, el)| {
            let set: HashSet<String> = HashSet::from_iter(el.into_iter().cloned());
            map.insert(i, set);
        });
        let mut result = Vec::new();
        for (key, value) in map.iter() {
            let mut flag = true;
            for i in 0..favorite_companies.len() {
                let other = map.get(&i).unwrap();
                if *key != i && value.is_subset(&other) {flag = false}
            }
            if flag {result.push(*key as i32)}
        }
        result.sort();
        result
    }
}
