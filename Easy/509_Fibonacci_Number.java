class Solution {
    public int fib(int n) {
        
        if (n == 0) return 0;
        int[] dp = new int[n+1];
        
        dp[1] = 1;
        
        for (int i = 0; i < n; i++) {
            dp[i+1] += dp[i];
            if (i < n-1) {
                dp[i+2] += dp[i];  
            }
        }
        
        return dp[n];
    }
}

/*
Runtime: 0 ms, faster than 100.00% of Java online submissions for Fibonacci Number.
Memory Usage: 39 MB, less than 90.15% of Java online submissions for Fibonacci Number.
*/
