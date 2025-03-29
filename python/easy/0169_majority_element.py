class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        current, count = nums[0], 0
        for el in nums:
            if el == current:
                count += 1
            else:
                count -= 1
            if count == 0:
                count = 1
                current = el
        return current
