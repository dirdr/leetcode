impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut need = vec![false; 366];
        let mut dp = vec![-1; 366];
        for &day in &days {
            need[day as usize] = true;
        }
        fn dfs(d: i32, d0: i32, costs: &Vec<i32>, need: &Vec<bool>, dp: &mut Vec<i32>) -> i32 {
            if d < d0 {
                return 0;
            }

            if !need[d as usize] {
                return dfs(d - 1, d0, costs, need, dp);
            }

            if dp[d as usize] != -1 {
                return dp[d as usize];
            }

            dp[d as usize] = (costs[0] + dfs(d - 1, d0, costs, need, dp))
                .min(costs[1] + dfs(d - 7, d0, costs, need, dp))
                .min(costs[2] + dfs(d - 30, d0, costs, need, dp));
            dp[d as usize]
        }

        let first_day = *days.first().unwrap() as i32;
        let last_day = *days.last().unwrap() as i32;

        dfs(last_day, first_day, &costs, &need, &mut dp)
    }
}
