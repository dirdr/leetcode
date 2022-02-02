class Solution {
    public List<List<Integer>> groupThePeople(int[] groupSizes) {
        List<List<Integer>> answer = new ArrayList<>();
        HashMap<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < groupSizes.length; i++) {
            map.put(i, groupSizes[i]);
        }
        
        while (true) {
            int min = Integer.MAX_VALUE;
            boolean all_done = true;
            for (int key: map.keySet()) {
                min = ((map.get(key) < min) && (map.get(key) != -1)) ? map.get(key): min;
                if (map.get(key) != -1) all_done = false;
            }
            if (all_done == true) return answer;
            int size = min;
            List<Integer> temp = new ArrayList<>();
            for (int i = 0; i < groupSizes.length; i++) {
                if (map.get(i) == size) {
                    temp.add(i);
                    map.put(i, -1);
                }
                if (temp.size() == size) {
                    break;
                }
            }
            answer.add(temp);
        }    
    }
}

/**
* Runtime: 342 ms, faster than 5.31% of Java online submissions for Group the People Given the Group Size They Belong To.
* Memory Usage: 109.6 MB, less than 5.20% of Java online submissions for Group the People Given the Group Size They Belong To.
* /
