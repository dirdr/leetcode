import heapq

class Solution:
    def putMarbles(self, weights: List[int], k: int) -> int:
        n = len(weights)
        minHeap, maxHeap = [], []
        minScore, maxScore = 0, 0
        for i in range(n - 1):
            val = weights[i] + weights[i + 1]
            heapq.heappush(min1Heap, val)
            heapq.heappush(maxHeap, -val)
        for i in range(k - 1):
            maxScore += heapq.heappop(maxHeap) * -1
            minScore += heapq.heappop(minHeap)
        return maxScore - minScore
