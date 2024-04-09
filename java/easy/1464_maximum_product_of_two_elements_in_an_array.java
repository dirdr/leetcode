class Solution {
    public int maxProduct(int[] nums) {
        // sort the Array such that nums[i-1] is the biggest and nums[i-2] the second biggest
        Arrays.sort(nums); 
        // if the Array length is greater than 1 we return nums[i-1]-1 * nums[i-2]-1 else 0
        return nums.length > 1 ? (nums[nums.length-1]-1)*(nums[nums.length-2]-1): 0;
    }
}

/**
 * Runtime: 1 ms, faster than 67.58% of Java online submissions for Maximum Product of Two Elements in an Array.
 * Memory Usage: 38.6 MB, less than 73.56% of Java online submissions for Maximum Product of Two Elements in an Array.
 */

