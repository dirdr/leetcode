class Solution {
    public int[] findErrorNums(int[] nums) {
        int[] result = new int[2];
        HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
        for (int num: nums) {
            map.put(num, map.getOrDefault(num, 0) + 1);
        }
        for (int key: map.keySet()) {
            if (map.get(key) == 2) {
                result[0] = key;
            }
        }
        for (int i = 1; i <= nums.length; i++) {
            if (!map.containsKey(i)) {
                result[1] = i;
            }
        }
        return result;
    }
}

/** 
 *Runtime: 15 ms, faster than 23.32% of Java online submissions for Set Mismatch.
 *Memory Usage: 40.5 MB, less than 73.08% of Java online submissions for Set Mismatch.
 */ 
