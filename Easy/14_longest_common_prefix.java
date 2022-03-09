class Solution {
    public String longestCommonPrefix(String[] strs) {
        
        StringBuilder sb = new StringBuilder();
        int obj = strs.length;
        int shortest = Integer.MAX_VALUE;
        
        for (String str: strs) {
            shortest = str.length() < shortest ? str.length(): shortest;
        }
        
        for (int i = 0; i < shortest; i++) {
            Set<Character> checker = new HashSet<>();
            char curr = '0';
            for (String str: strs) {
                curr = str.charAt(i);
                checker.add(curr);
            }
            if (checker.size() == 1) {
                sb.append(curr);
            } else break;
        }
        
        return sb.length() == 0 ? "": sb.toString();
    }
}

/*
* Runtime: 4 ms, faster than 30.80% of Java online submissions for Longest Common Prefix.
* Memory Usage: 42.5 MB, less than 20.55% of Java online submissions for Longest Common Prefix.
*/
