use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut coordinate = (0, 0);
        let mut set = HashSet::new();
        let mut action = HashMap::from([
            ('N', (0, 1)),
            ('S', (0, -1)),
            ('E', (1, 0)),
            ('W', (-1, 0)),
        ]);
        set.insert(coordinate);
        for ch in path.chars() {
            coordinate.0 += action.get(&ch).unwrap().0;
            coordinate.1 += action.get(&ch).unwrap().1;
            if set.contains(&coordinate) {return true} else {set.insert(coordinate);}
        }
        false
    }
}
