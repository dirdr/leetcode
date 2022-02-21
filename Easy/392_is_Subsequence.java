class Solution {
    public boolean isSubsequence(String s, String t) {
  
        
        if (s.length() == 0) return true;
        
        StringBuilder sb = new StringBuilder();
        int looking_for = 0;
        
        for (int i = 0; i < t.length(); i++) {
            char lfc = s.charAt(looking_for);
            if (t.charAt(i) == lfc) {
                sb.append(t.charAt(i));
                looking_for++;
            }
            if (sb.length() == s.length()) return true;
        }
        
        System.out.println(sb.toString());
        return false;        
    }
}

/**
* Runtime: 1 ms, faster than 87.30% of Java online submissions for Is Subsequence.
* Memory Usage: 39.9 MB, less than 42.58% of Java online submissions for Is Subsequence.
*/
