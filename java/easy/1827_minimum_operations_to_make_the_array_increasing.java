class Solution {
    public int minOperations(int[] nums) {
        int ans = 0;
        int index = 0;
        while (index < nums.length-1) {
            if (nums[index] >= nums[index+1]) {
                ans++;
                nums[index+1]++;
            } else {
                index++;
            }
        }
        return ans;
    }
}

/**
 * Runtime: 464 ms, faster than 5.72% of Java online submissions for Minimum Operations to Make the Array Increasing.
 * Memory Usage: 39.5 MB, less than 60.17% of Java online submissions for Minimum Operations to Make the Array Increasing.
 */
