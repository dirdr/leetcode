class Solution {
    public String[] uncommonFromSentences(String s1, String s2) {
        
        ArrayList<String> helper = new ArrayList<>();
        HashMap<String, Integer> map1 = new HashMap<>();        
        HashMap<String, Integer> map2 = new HashMap<>();
  
        for (String str: s1.split(" ")) {
             map1.put(str, map1.getOrDefault(str, 0)+1);
        }
        
        for (String str: s2.split(" ")) {
            map2.put(str, map2.getOrDefault(str, 0)+1);
        }
        
        for (String key: map1.keySet()) {
            if (map1.get(key) == 1 && map2.get(key) == null) {
                helper.add(key);
            }
        }
        
        for (String key: map2.keySet()) {
            if (map2.get(key) == 1 && map1.get(key) == null) {
                helper.add(key);
            }
        }
        
        String[] answer = new String[helper.size()];
        for (int i = 0; i < helper.size(); i++) {
            answer[i] = helper.get(i);
        }
        return answer;
    }
}

/*
* Runtime: 3 ms, faster than 78.75% of Java online submissions for Uncommon Words from Two Sentences.
* Memory Usage: 43.2 MB, less than 10.57% of Java online submissions for Uncommon Words from Two Sentences.
*/
