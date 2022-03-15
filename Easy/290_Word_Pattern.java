class Solution {
    public boolean wordPattern(String pattern, String s) {
        HashMap<Character, String> map = new HashMap<>();
        char[] patt_arr = pattern.toCharArray();
        String[] s_arr = s.split(" ");
        if (patt_arr.length != s_arr.length) return false;
        for (int i = 0; i < pattern.length(); i++) {
            char ch = patt_arr[i];
            String w = s_arr[i];
            if (map.get(ch) == null) {
                for (char key: map.keySet()) {
                    if (map.get(key).equals(w)) return false;
                }
                map.put(ch, w);
            } else {
                if (!map.get(ch).equals(w)) {
                    return false;
                }
            }
        }
        return true;
    }
}

/*
* Runtime: 1 ms, faster than 91.61% of Java online submissions for Word Pattern.
* Memory Usage: 41.9 MB, less than 11.89% of Java online submissions for Word Pattern.
*/
