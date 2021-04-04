class Solution {
    public String truncateSentence(String s, int k) {
        String[] splited = s.split(" ");
        String result = "";
        for (int i = 0; i < k; i++) {
            result += splited[i];
            if (i != k-1) {
                result += " ";
            }
        }
        return result;
    }
}

/**
 * Runtime: 2 ms, faster than 100.00% of Java online submissions for Truncate Sentence.
 * Memory Usage: 39 MB, less than 50.00% of Java online submissions for Truncate Sentence.
 */
