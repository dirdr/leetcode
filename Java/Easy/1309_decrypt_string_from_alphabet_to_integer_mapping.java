class Solution {
    public String freqAlphabets(String s) {
        HashMap<String, Character> map = new HashMap<String, Character>();
        int number = 0;
        for (char ch: "abcdefghi".toCharArray()) {
            map.put(String.valueOf(++number), ch);
        }
        for (char ch: "jklmnopqrstuvwxyz".toCharArray()) {
            String put = (String.valueOf(++number) + "#");
            map.put(put, ch);
        }
        int indice = 0;
        StringBuilder sb = new StringBuilder();
        for (int i = s.length()-1; i >= 0; i-= indice) {
            char curr = s.charAt(i);
            indice = curr == '#' ? 3: 1;
            System.out.println(indice);
            if (curr == '#') {
                sb.append(map.get(s.substring(i-2, i+1)));
            } else {
                sb.append(map.get(s.substring(i, i+1)));
            }
        }    
        return sb.reverse().toString();
    }   
}
/**
 *Runtime: 11 ms, faster than 9.41% of Java online submissions for Decrypt String from Alphabet to Integer Mapping.
 *Memory Usage: 38.7 MB, less than 42.09% of Java online submissions for Decrypt String from Alphabet to Integer Mapping.
 */
