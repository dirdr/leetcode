class Solution {
    public double average(int[] salary) {
        Arrays.sort(salary);
        int sum = 0;
        for (int i = 1; i < salary.length-1; i++) {
            sum += salary[i];
        }
        double result = (sum/((double)salary.length-2));
        return result;
   
    }
}

/*
* Runtime: 1 ms, faster than 36.26% of Java online submissions for Average Salary Excluding the Minimum and Maximum Salary.
* Memory Usage: 41.8 MB, less than 49.02% of Java online submissions for Average Salary Excluding the Minimum and Maximum Salary.
*/
