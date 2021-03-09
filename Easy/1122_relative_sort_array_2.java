class Solution {
    public int[] relativeSortArray(int[] arr1, int[] arr2) {
        HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
        for (int num: arr2) {
        map.put(num, 0);
        }   
        int[] result = new int[arr1.length];
        int index = 0;
        for (int num: arr1) {
            if (map.containsKey(num)) {
                map.put(num, map.get(num) + 1);
            } else {
                result[arr1.length-1-index++] = num;
            }
        }
        index = 0;
        for (int num: arr2) {
            int count = map.get(num);
            for (int i = index; i < index + count; i++) {
                result[i] = num;
            }
            index += count;
        }
        Arrays.sort(result, index, result.length);
        return result;
    }
        
}
/**
 * Runtime: 1 ms, faster than 80.04% of Java online submissions for Relative Sort Array.
 * Memory Usage: 39.2 MB, less than 42.02% of Java online submissions for Relative Sort Array.
 */


