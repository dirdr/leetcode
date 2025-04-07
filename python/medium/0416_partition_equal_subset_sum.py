class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        numsSum = sum(nums)
        if numsSum % 2 != 0:
            return False

        target = numsSum // 2
        dp = set()
        dp.add(0)
    
        for i in range(len(nums) - 1, -1, -1):
            nextDp = set()
            for s in dp:
                if nums[i] + s == target:
                    return True
                nextDp.add(nums[i] + s)
                nextDp.add(s)
            dp = nextDp
            
        return True if target in dp else False
