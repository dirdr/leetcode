class Solution:
    def repairCars(self, ranks: List[int], cars: int) -> int:
        left, right = 1, min(ranks) * (cars ** 2)
        result = right
        while left <= right:
            middle = left + (right - left) // 2
            if self.isFeasible(ranks, cars, middle):
                result = middle
                right = middle - 1
            else:
                left = middle + 1
        return result

    def isFeasible(self, ranks: List[int], cars: int, time: int) -> bool:
        capacity = 0
        for r in ranks:
            capacity += int((time / r) ** 0.5)
            if capacity >= cars:
                return True
        return False
