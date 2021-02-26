class Solution {
    public String destCity(List<List<String>> paths) {
        HashMap<String, Integer> map = new HashMap<String, Integer>();
        for (List<String> list: paths) {
            String first = list.get(0);
            String second = list.get(1);
            map.put(first, map.getOrDefault(first, 0) + 1);
            if (map.containsKey(second)) {
                map.put(second, map.get(second) + 1);
            } else {
                map.put(second, 0);
            }
        }
        for (String str: map.keySet()) {
            if (map.get(str) == 0) {
                return str;
            }
        }
        return null;
    }
}
