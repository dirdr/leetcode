class Solution {
    public List<String> stringMatching(String[] words) {
        List<String> list = new ArrayList<String>();
        for (int i = 0; i < words.length; i++) {
            String curr = words[i];
            for (int j = 0; j < words.length; j++) {
                String iterate = words[j];
                if (curr.contains(iterate) && curr != iterate) {
                    if (!list.contains(iterate)) {
                        list.add(iterate);
                    }
                }
            }
        }
        return list;
    }
}
