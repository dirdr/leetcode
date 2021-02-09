class Solution {
    public int[] intersect(int[] nums1, int[] nums2) {
        
        HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
        List<Integer> list = new ArrayList<Integer>();
        
        for (int num: nums1) {
            map.put(num, map.getOrDefault(num, 0) + 1);
        }
    
        for (int num : nums2) {
            if (map.containsKey(num) && map.get(num) > 0) {
                list.add(num);
                map.put(num, map.get(num) - 1);
            }
        }
        int len = list.size();
        int[] result = new int[len];
        for (int i = 0; i < len; i++) {
            result[i] = list.get(i);
        }
        return result;
    }
}
