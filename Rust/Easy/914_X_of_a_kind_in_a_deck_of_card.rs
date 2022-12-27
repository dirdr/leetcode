use std::collections::HashMap;

impl Solution {
    pub fn get_div(a: &i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 2..=*a {
            if *a % i == 0 {
                result.push(i);
            }
        }
        result
    }

    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        if (deck.len() == 1) {return false;}
        let mut map = HashMap::new();
        deck.iter().for_each(|card| {
            map.entry(card).and_modify(|e| {*e += 1}).or_insert(1);
        });
        let min = *map
            .iter()
            .filter(|el| *el.1 != 1)
            .min_by(|a, b| {a.1.cmp(b.1)}).unwrap().1;
        let mut result = true;
        for value in Solution::get_div(&min).iter() {
            let mut flag = true;
            map.iter().for_each(|key_value| {
                if *key_value.1 % value != 0 {
                    flag = false;
                }
                if (flag == false) {result = false}
            });
            if flag == true {return true}
        }
        result
    }
}
