class Solution {
    public int minSteps(String s, String t) {
        int step = 0;
        HashMap<Character, Integer> map = new HashMap<Character, Integer>();
        for (char ch: s.toCharArray()) {
            map.put(ch, map.getOrDefault(ch, 0) + 1);
        }
        for (char ch: t.toCharArray()) {
            if (map.containsKey(ch) && map.get(ch) > 0) {
                map.put(ch, map.get(ch) - 1);
            } else {
                step++;
            }
        }
        return step;
    }
}

/**
*Runtime: 53 ms, faster than 18.28% of Java online submissions for Minimum Number of Steps to Make Two *Strings Anagram.
*Memory Usage: 39.6 MB, less than 64.67% of Java online submissions for Minimum Number of Steps to *Make Two Strings Anagram.
*/
