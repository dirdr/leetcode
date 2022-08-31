class Solution {
    public int findCenter(int[][] edges) {
        
        HashMap<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < edges.length; i++) {
            int a = edges[i][0];
            int b = edges[i][1];
            map.put(a, map.getOrDefault(a, 0)+1);
            map.put(b, map.getOrDefault(b, 0)+1);
        }
        int max = 0;
        for (int key: map.keySet()){
            max = map.get(key) > max ? key: max;
        }
        return max;     
    }
}

/**
* Runtime: 62 ms, faster than 6.02% of Java online submissions for Find Center of Star Graph.
* Memory Usage: 116.1 MB, less than 5.03% of Java online submissions for Find Center of Star Graph.
*/
