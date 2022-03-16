class Solution {
    public String[] findRestaurant(String[] list1, String[] list2) {
        
        HashMap<String, Integer> map = new HashMap<>();
        
        String longest[];
        String shortest[];
        
        if (list1.length >= list2.length) {
            longest = list1;
            shortest = list2;
        } else {
            longest = list2;
            shortest = list1;
        }
        
        for (int i = 0; i < longest.length; i++) {
            map.put(longest[i], i);
        }
        
        int min = Integer.MAX_VALUE;
        
        ArrayList<String> helper = new ArrayList<>();
        
        for (int i = 0; i < shortest.length; i++) {
            if (map.get(shortest[i]) != null) {
                int sum = map.get(shortest[i])+i;
                if (sum == min) {
                    helper.add(shortest[i]);
                } else if (sum < min) {
                    min = sum;
                    helper.clear();
                    helper.add(shortest[i]);
                }
            }
        }
        
        String[] answer = new String[helper.size()];
        for (int i = 0; i < answer.length; i++) {
            answer[i] = helper.get(i);
        }
        return answer;
    }
}

/*
* Runtime: 15 ms, faster than 43.80% of Java online submissions for Minimum Index Sum of Two Lists.
* Memory Usage: 54.5 MB, less than 41.03% of Java online submissions for Minimum Index Sum of Two Lists.
*/
