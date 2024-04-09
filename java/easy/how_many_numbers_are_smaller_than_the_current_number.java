class Solution {
    public int[] smallerNumbersThanCurrent(int[] nums) {    
        int[] result = new int[nums.length];
        for(int i = 0; i < nums.length; i++){
            int temp = 0;
            for(int y = 0; y < nums.length; y++){
                if(nums[i] > nums[y])
                    temp++;
            }
            result[i] = temp;
        }
        return result;
    }
}
