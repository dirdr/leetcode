class Solution:
    def minCapability(self, nums: List[int], k: int) -> int:
        left, right = 1, max(nums)
        ans = right
        while left <= right:
            middle = left + (right - left) // 2
            if self.canRob(nums, middle, k):
                right = middle - 1
                ans = middle
            else:
                left = middle + 1
        return ans


    def canRob(self, nums: List[int], mid: int, k: int) -> bool:
        count = 0
        n = len(nums)
        i = 0
        while i < n:
            if nums[i] <= mid:
                count += 1
                i += 1
            i += 1
        return count >= k
