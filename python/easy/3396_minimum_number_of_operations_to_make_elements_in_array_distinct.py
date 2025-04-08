class Solution:
    def minimumOperations(self, nums: List[int]) -> int:
        def distincts(array: List[int]):
            if len(array) == 0:
                return True

            distinct = set()
            for el in array:
                if el in distinct:
                    return False
                distinct.add(el)
            return True
        
        left, right = 0, len(nums) // 3 + 1
        while left <= right:
            middle = left + (right - left) // 2
            if distincts(nums[middle * 3::]):
                right = middle - 1
            else:
                left = middle + 1
        return left
