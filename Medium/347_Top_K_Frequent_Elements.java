class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        
        HashMap<Integer, Integer> map = new HashMap<>();
        
        for (int num: nums) {
            map.put(num, map.getOrDefault(num, 0)+1);
        }
        
        ArrayList<Integer> list = new ArrayList<>();
        Set<Integer> banned = new HashSet<>();

        for (int count = 0; count < k; count++) {
            int max = 0;
            int max_key = 0;
            for (int key: map.keySet()) {
                int val = map.get(key);
                if (val > max && !banned.contains(key)) {
                    max = val;
                    max_key = key;
                }
            }
            list.add(max_key);
            banned.add(max_key);
        }
        
        int[] answer = new int[list.size()];
        for (int i = 0; i < list.size(); i++) {
            answer[i] = list.get(i);
        }
        return answer;
    }
}

/*
* Runtime: 160 ms, faster than 5.04% of Java online submissions for Top K Frequent Elements.
* Memory Usage: 67 MB, less than 5.25% of Java online submissions for Top K Frequent Elements.
*/
