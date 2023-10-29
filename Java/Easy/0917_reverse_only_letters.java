class Solution {
    public String reverseOnlyLetters(String S) {
        List<Character> list = new ArrayList<Character>();
        for (char ch: S.toCharArray()) {
            if (Character.isLetter(ch)) {
                list.add(ch);
            }
        }
        Collections.reverse(list);
        for (int i = 0; i < S.length(); i++) {
            char curr = S.charAt(i);
            if (!Character.isLetter(curr)) {
                list.add(i, curr);
            }
        }
        return list.stream().map(Object::toString).collect(Collectors.joining());
    }
}
/**
 *Runtime: 4 ms, faster than 10.97% of Java online submissions for Reverse Only Letters.
 *Memory Usage: 37.8 MB, less than 23.89% of Java online submissions for Reverse Only Letters.
 */
