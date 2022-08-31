class Solution {
      
    public int rob(int[] nums) {
        
        int l = nums.length;
        
        if (l == 1) return nums[0];
        if (l == 2) return Math.max(nums[0], nums[1]);
        
        int[] dp = new int[l];
        
        dp[0] = nums[0];
        dp[1] = nums[1];
        dp[2] = dp[0] + nums[2];

        
        for (int i = 3; i < nums.length; i++) {
            dp[i] = Math.max(dp[i-2], dp[i-3]) + nums[i];
        }
        
        return Math.max(dp[l-1], dp[l-2]);
        
    }
        
}

/**
Runtime: 0 ms, faster than 100.00% of Java online submissions for House Robber.
Memory Usage: 39.6 MB, less than 91.54% of Java online submissions for House Robber.
*/
