class Solution {
public:
    bool isIsomorphic(string s, string t) {
        map<char, char> map;
        set<char> set;
        for (int i = 0; i < s.length(); i++) {
            char sc = s[i];
            char tc = t[i];
            if (map.find(sc) != map.end()) {
                if (map[sc] != tc) return false;
            } else {
                if (set.find(tc) != set.end()) return false;
                set.insert(tc);
                map.insert({sc, tc});
            }
        }
        return true;
    }
};
