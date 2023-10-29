class Solution {
    public boolean isSubsequence(String s, String t) {

        if (s.length() == 0) return true;
        int index = 0;

        for (int i = 0; i < t.length(); i++) {
           char curr = t.charAt(i);
           if (s.charAt(index) == curr) {
               index++;
           }
           if (index == s.length())  return true;
        } 

        return false; 
    }
}

/**
Runtime: 2 ms, faster than 46.59% of Java online submissions for Is Subsequence.
Memory Usage: 41.6 MB, less than 77.19% of Java online submissions for Is Subsequence.
*/
