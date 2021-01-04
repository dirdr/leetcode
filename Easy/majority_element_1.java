class Solution {
    public int majorityElement(int[] nums) {
        
        if (nums.length < 2) {
            return nums[0];
        }
        
        HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
        
        for(int i = 0; i < nums.length; i++) {
            
            if (map.containsKey(nums[i])) { 
                    int count = map.get(nums[i]) + 1;
                    if (count > nums.length/2) {
                        return nums[i];
                    } else
                        map.put(nums[i], count); 
            } 
            else {
                map.put(nums[i],1); 
            }
        }
        return 0;
    }
}
