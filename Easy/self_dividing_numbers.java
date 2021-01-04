class Solution {
    public List<Integer> selfDividingNumbers(int left, int right) {
        ArrayList<Integer> arrayList = new ArrayList();
        for (int i = left; i <= right; i++) {
            int tempInt = i;
            boolean flag = true;
            
            for (int j = 0; j < String.valueOf(tempInt).length(); j++) {
                char tempDigit = String.valueOf(tempInt).charAt(j);
                if (tempDigit == '0') {
                    flag = false;
                    break;
                }
                if (tempInt % Character.getNumericValue(tempDigit) != 0) {
                    flag = false;
                    break;
                }
            }
            if (flag == true) {
                arrayList.add(tempInt);
            }
        }
        return arrayList;
    }
}
