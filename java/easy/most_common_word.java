class Solution {
    public String mostCommonWord(String paragraph, String[] banned) {
        String[] list = paragraph.replaceAll("[^A-Za-z]"," ").toLowerCase().split(" ");
        Set<String> bannedSet = new HashSet<String>();
        for (String str: banned) {
            bannedSet.add(str);
        }
        Map<String, Integer> map = new HashMap<String, Integer>();
        for (String str: list) {
            if (!bannedSet.contains(str)) { 
                map.put(str, map.getOrDefault(str, 0) + 1);               
            }
        }
        //max algorithm adapted to the map keySet()
        String max = "";
        for (String word: map.keySet()) {
            if (max.equals("")) {
                max = word;
            }
            max = map.get(word) > map.get(max) ? word : max;
        }
        return max;
    }
}
