from collections import defaultdict

class Solution:
    def longestPalindrome(self, s: str) -> int:
        freqs = defaultdict(int)
        for ch in s:
            freqs[ch] += 1
        oneTaken = False
        answer = 0
        for freq in freqs.values():
            if freq % 2 == 0:
                answer += freq
            else:
                if freq > 2:
                    answer += freq - 1
                if not oneTaken:
                    answer += 1
                    oneTaken = True
        return answer
