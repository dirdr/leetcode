# The isBadVersion API is already defined for you.
# def isBadVersion(version: int) -> bool:

class Solution:
    def firstBadVersion(self, n: int) -> int:
        left, right = 1, n
        result = 0
        while left <= right:
            middle = left + (right - left) // 2
            if not isBadVersion(middle):
                result = middle
                left = middle + 1
            else:
                right = middle - 1
        return result + 1
