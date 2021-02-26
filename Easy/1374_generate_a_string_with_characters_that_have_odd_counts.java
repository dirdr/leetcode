class Solution {
    public String generateTheString(int n) {
       char[] answer = new char[n];
        Arrays.fill(answer, 'a');
        if (n % 2 == 0) {
            answer[0] = 'b';
        }
        return new String(answer);
    }
}
