class Solution {
    public String reformatDate(String date) {
        String[] splited = date.split(" ");
        HashMap<String, String> month = new HashMap<String, String>();
        String[] monthArray = {"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"};
        int i = 0;
        for (String str: monthArray) {
            if (i >= 9) {
                month.put(str, String.valueOf(++i));
            } else {
                String temp = "0" + String.valueOf(++i);
                month.put(str, temp);
            }
        }
        String answer = "";
        String number = "";
        
        for (int j = 0; j < splited[0].length(); j++) {
            char curr = date.charAt(j);
            if (Character.isDigit(curr)) {
                number += curr;
            }
        }
        if (Integer.parseInt(number) < 10) {
            number = "0" + number;
        }
        answer += splited[2];
        answer += "-";
        answer += String.valueOf(month.get(splited[1]));
        answer += "-";
        answer += number;
        return answer;
    }
}

/**
 *Runtime: 7 ms, faster than 38.83% of Java online submissions for Reformat Date.
 *Memory Usage: 37.4 MB, less than 76.33% of Java online submissions for Reformat Date.
 */
