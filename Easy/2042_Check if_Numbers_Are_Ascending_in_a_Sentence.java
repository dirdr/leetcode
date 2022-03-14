class Solution {
    public boolean areNumbersAscending(String s) {
        int prev = -1;
        for (String str: s.split(" ")) {
            if (Character.isDigit(str.charAt(0))) {
                int num = Integer.parseInt(str);
                if (num <= prev) return false;
                else prev = num;                
            }
        }
        return true;
    }
}

/*
* Runtime: 2 ms, faster than 75.09% of Java online submissions for Check if Numbers Are Ascending in a Sentence.
* Memory Usage: 42.4 MB, less than 27.30% of Java online submissions for Check if Numbers Are Ascending in a Sentence.
*/
