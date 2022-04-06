class Solution {
    public int minCostClimbingStairs(int[] cost) {
    
        int[] dp = new int[cost.length];
        
        dp[0] = cost[0];
        dp[1] = cost[1];
        
        for (int i = 2; i < cost.length; i++) {
            dp[i] += Math.min(dp[i-1], dp[i-2]) + cost[i];
        }

        return Math.min(dp[cost.length-1], dp[cost.length-2]);
    }
}

/**
* Runtime: 1 ms, faster than 86.34% of Java online submissions for Min Cost Climbing Stairs.
* Memory Usage: 43.3 MB, less than 70.68% of Java online submissions for Min Cost Climbing Stairs.
*/
