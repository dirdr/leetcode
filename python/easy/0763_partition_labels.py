from collections import defaultdict

class Solution:
    def partitionLabels(self, s: str) -> List[int]:
        n = len(s)
        lastIndex = defaultdict()
        for i, ch in enumerate(s):
            lastIndex[ch] = i

        size, validUntil, answer = 0, lastIndex[s[0]], []
        for i, ch in enumerate(s):
            size += 1
            validUntil = max(validUntil, lastIndex[ch])
            if i == validUntil:
                answer.append(size)
                size = 0
        return answer
