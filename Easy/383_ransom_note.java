class Solution {
    public boolean canConstruct(String ransomNote, String magazine) {
        
        HashMap<Character, Integer> map = new HashMap<Character, Integer>();
        
        for (char ch: magazine.toCharArray()) {
            map.put(ch, map.getOrDefault(ch, 0) + 1);
        }
        
        boolean result = true;
        for (char ch: ransomNote.toCharArray()) {
            if (!map.containsKey(ch) || map.get(ch) == 0) {
                return false;
            } else {
                map.put(ch, map.get(ch) -1);
            }
        }
        return true;
    }
}

/**
 * Runtime: 9 ms, faster than 34.18% of Java online submissions for Ransom Note.
 * Memory Usage: 39.1 MB, less than 84.24% of Java online submissions for Ransom Note.
 */
