class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        
        List<List<String>> answer = new ArrayList<>();
        HashMap<String, ArrayList<String>> map = new HashMap<>();
        
        for (String str: strs) {
            char[] ca = str.toCharArray();
            Arrays.sort(ca);
            String sorted = new String(ca);
            if (map.containsKey(sorted)) {
                 map.get(sorted).add(str);
            } else {
                ArrayList<String> temp = new ArrayList<>();
                temp.add(str);
                map.put(sorted, temp);
            }
        }
        
        for (String key: map.keySet()) {
            answer.add(map.get(key));
        }
        
        return answer;
    }
}
