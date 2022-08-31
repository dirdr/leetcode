class Solution {
    public int findMaxConsecutiveOnes(int[] nums) {
        if (nums.length == 0) {
            return 0;
        }
        int max = 0;
        ArrayList<Integer> list = new ArrayList<Integer>();
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] == 1) {
                max++;
                if (i+1 == nums.length) {
                    list.add(max);
                    break;
                }
            } else if (nums[i] == 0) {
                list.add(max);
                max = 0;
            } 
         }
    return Collections.max(list);
    }
}
