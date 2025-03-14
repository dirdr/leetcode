class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        freq = [0 for ch in range(26)]
        for ch in s:
            freq[ord(ch) - ord('a')] += 1
        for ch in t:
            freq[ord(ch) - ord('a')] -= 1
        return all(val == 0 for val in freq)
