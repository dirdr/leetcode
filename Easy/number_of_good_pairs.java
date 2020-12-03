class Solution {
    public int numIdenticalPairs(int[] nums) {
        int count = 0;
        for(int i = 0; i < nums.length - 1; i++){
            for(int y = i+1; y < nums.length; y++){
                if(nums[i] == nums[y]){
                    count++;
                }
            }
        }
        return count;
    }
}
