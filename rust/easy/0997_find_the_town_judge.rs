use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.len() == 0 {return 1;}
        let mut map = HashMap::new();
        trust.clone().iter().for_each(|t| {
            map.entry(t[1]).and_modify(|e| {*e += 1}).or_insert(1);
        });
        match map.iter().find(|e| *e.1 == n-1) {
            Some(value) => if trust.iter().filter(|e| e[0] == *value.0).next() == None {return *value.0;} else {return -1;},
            None => return -1,
        };
    }
}
