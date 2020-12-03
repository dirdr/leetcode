class Solution {
    public int[] runningSum(int[] nums) {
        int[] result = new int[nums.length];
        int temp = 0;
        for(int i = 0; i < result.length; i++){
            temp += nums[i];
            result[i] = temp;
        }
        return result; 
    }
}
