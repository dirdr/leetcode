class Solution {
    public boolean checkIfPangram(String sentence) {
        Set<Character> set = new HashSet<Character>();
        for (char ch: sentence.toCharArray()){
            set.add(ch);
        }
        return set.size() == 26;
    }
}

/**
 * Runtime: 2 ms, faster than 56.58% of Java online submissions for Check if the Sentence Is Pangram.
 * Memory Usage: 37.2 MB, less than 59.21% of Java online submissions for Check if the Sentence Is Pangram.
 */
