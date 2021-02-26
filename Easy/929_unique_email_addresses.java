class Solution {
    public int numUniqueEmails(String[] emails) {
        HashSet<String> set = new HashSet();
        for (String adress: emails) {
            String[] separated = adress.split("@");
            String first = separated[0];
            if (first.contains("+")) {
                first = first.substring(0, first.indexOf('+'));
            }
            first = first.replace(".", "");
            set.add(first + "@" + separated[1]);

        }
        return set.size();
    }
}
