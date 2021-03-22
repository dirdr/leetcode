class Solution {
    public int maxAscendingSum(int[] nums) {
    
        int max = nums[0];
        int sum = nums[0];
        for (int i = 1; i < nums.length; i++) {
            // if the current number is greater than the previous number we can continue to run the sum
            //else we set the seum value to be the nums[i] value ready to start again
            sum = nums[i] > nums[i-1]  ? sum+nums[i]: nums[i];
            //max algorithm
            max = sum > max ? sum: max;
        }
        return max;
    }
}
/**
 * Runtime: 0 ms, faster than 100.00% of Java online submissions for Maximum Ascending Subarray Sum.
 * Memory Usage: 36.6 MB, less than 100.00% of Java online submissions for Maximum Ascending Subarray Sum. 
 */


