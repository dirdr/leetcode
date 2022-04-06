class Solution {
    public int tribonacci(int n) {
        
        if (n == 0) return 0;
        if (n == 1) return 1;
        
        int[] dp = new int[n+1];
        dp[1] = 1;
        
        for (int i = 1; i < n; i++) {
            System.out.println(Arrays.toString(dp));
            dp[i+1] += dp[i];
            if (i < n - 1) dp[i+2] += dp[i];
            if (i < n - 2) dp[i+3] += dp[i];
        }
        
        return dp[n];                
    }
}

/**
Runtime: 4 ms, faster than 11.24% of Java online submissions for N-th Tribonacci Number.
Memory Usage: 39.1 MB, less than 83.74% of Java online submissions for N-th Tribonacci Number.
*/
