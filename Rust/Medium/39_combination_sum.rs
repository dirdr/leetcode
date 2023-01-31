impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut used: Vec<Vec<i32>> = vec![];
        Self::dfs(&candidates, target, &mut result, &mut vec![], &mut used);
        result
    }

    pub fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        used: &mut Vec<Vec<i32>>,
    ) -> () {
        let sum: i32 = path.iter().sum();
        if (sum > target) {return}
        if (sum == target) {
            let mut temp = path.clone();
            temp.sort();
            if (!used.contains(&temp)) {
                used.push(temp);
                result.push(path.clone());
                return;
            } 
        }
        for i in 0..candidates.len() {
            path.push(candidates[i]);
            Self::dfs(candidates, target, result, path, used);
            path.pop();
        }
    }
}
