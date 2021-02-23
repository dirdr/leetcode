class Solution {
    public String reverseWords(String s) {
        String[] splited = s.split(" ");
        String result = "";
        for (int i = 0; i < splited.length; i++) {
            String reversed = "";
            char[] sToChar = splited[i].toCharArray();
            for (int j = sToChar.length-1; j >= 0; j--) {
                reversed += sToChar[j];
            }
            result += reversed;
            if (i != splited.length-1) {
                result += " ";
            }
        }
        return result;     
    }
}
