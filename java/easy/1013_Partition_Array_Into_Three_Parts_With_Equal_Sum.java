class Solution {
    public boolean canThreePartsEqualSum(int[] arr) {
        
        int sum = 0;
        
        for (int num: arr) sum += num;
        
        if (sum % 3 != 0) {
            return false;
        }
        
        int target_sum = sum/3;
        int part = 0;
        int partial_sum = 0;
        
        for (int i = 0; i < arr.length; i++) {
            partial_sum += arr[i];
            if (partial_sum == target_sum) {
                part++;
                partial_sum = 0;
            }
        }
        
        return part >= 3;
    }
}

/*
* Runtime: 2 ms, faster than 81.32% of Java online submissions for Partition Array Into Three Parts With Equal Sum.
* Memory Usage: 59.5 MB, less than 61.54% of Java online submissions for Partition Array Into Three Parts With Equal Sum.
*/
