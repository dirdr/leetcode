class Solution {
    public int findSpecialInteger(int[] arr) {
        HashMap<Integer, Integer> map = new HashMap<>();
        for (int num: arr) {
            map.put(num, map.getOrDefault(num, 0)+1);
        }
        for (int key: map.keySet()) {
            if ((float)map.get(key)/arr.length > 0.25) return key;
        }
        return -1;
    }
}

/**
 * Runtime: 31 ms, faster than 8.09% of Java online submissions for Element Appearing More Than 25% In Sorted Array.
 * Memory Usage: 43 MB, less than 52.88% of Java online submissions for Element Appearing More Than 25% In Sorted Array.
*/
