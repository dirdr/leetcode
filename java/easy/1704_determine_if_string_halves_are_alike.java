class Solution {
    public boolean halvesAreAlike(String s) {
        HashSet<Character> set = new HashSet();
        String vowel = "aeiouAEIOU";
        for (char ch: vowel.toCharArray()) {
            set.add(ch);
        }
        int count1, count2;
        count1 = count2 = 0;
        for (int i = 0; i < s.length()/2; i++) {
            if (set.contains(s.charAt(i))) {
                count1++;
            }
            if (set.contains(s.charAt(s.length()-1-i))) {
                count2++;
            }
        }
        return count1 == count2;
    }
}
