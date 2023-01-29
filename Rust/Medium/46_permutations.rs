impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        pub fn dfs(
            nums: &Vec<i32>,
            result: &mut Vec<Vec<i32>>,
            path: &mut Vec<i32>,
            used: &mut Vec<bool>
        ) -> () {
            if (path.len() == nums.len()) {
                result.push(path.clone());
                return;
            }
            for i in 0..nums.len() {  
                if !used[i] {
                    path.push(nums[i]);
                    used[i] = true;
                    dfs(nums, result, path, used);
                    used[i] = false;
                    path.pop();
                }
            }
        }
        let mut result: Vec<Vec<i32>> = vec![];
        let mut used: Vec<bool> = vec![false; nums.len()];
        dfs(&nums, &mut result, &mut vec![], &mut used);
        result
    }
}
