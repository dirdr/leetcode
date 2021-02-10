class Solution {
    public int[] decompressRLElist(int[] nums) {
        List<Integer> result = new ArrayList<Integer>();
        for (int i = 0; i < nums.length; i+=2) {
            int freq = nums[i];
            int val = nums[i+1];
            for (int j = 0; j < freq; j++) {
                result.add(val);
            }
        }
        int[] resultArr = new int[result.size()];
        for (int i = 0; i < result.size(); i++) {
            resultArr[i] = result.get(i);
        }
        return resultArr;
    }
}
