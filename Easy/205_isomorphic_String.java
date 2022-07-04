class Solution {
    public boolean isIsomorphic(String s, String t) {
        
        HashMap<Character, Character> map = new HashMap<>();
        HashSet<Character> set = new HashSet();
        
        for (int i = 0; i < s.length(); i++) {
            if (map.containsKey(s.charAt(i))) {
                if (map.get(s.charAt(i)) != t.charAt(i)) return false;
            } else {
                if (set.contains(t.charAt(i))) return false;
                map.put(s.charAt(i), t.charAt(i));
                set.add(t.charAt(i)); 
            }
        }
        
        return true;
    }
}

/**
Runtime: 12 ms, faster than 52.54% of Java online submissions for Isomorphic Strings.
Memory Usage: 42.8 MB, less than 72.77% of Java online submissions for Isomorphic Strings.
*/
