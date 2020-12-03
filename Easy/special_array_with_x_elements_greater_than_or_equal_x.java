class Solution {
    public int specialArray(int[] nums) {
        int max = 0;
        max = nums[0];
        for(int i = 0; i < nums.length; i++){
            if(nums[i] >= max)
                max = nums[i];
        }
        int temp = 0;
        for(int i = 0; i <= max; i++){
            for(int j = 0; j < nums.length; j++){
                if(nums[j] >= i)
                    temp++;
                    
            }
            if(temp == i)
                return i;
            else
                temp = 0;
        }
        return -1;   
    }
}
