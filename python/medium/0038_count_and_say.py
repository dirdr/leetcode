class Solution:
    def countAndSay(self, n: int) -> str:
        def aggregateFreq(n: str) -> List[int]:
            curr, count = n[0], 1
            answer = []
            for i in range(1, len(n)):
                if n[i] == curr:
                    count += 1
                else:
                    answer.extend([count, int(curr)])
                    curr, count = n[i], 1
            answer.extend([count, int(curr)])
            return answer

        curr = "1"
        for i in range(0, n - 1):
            curr = ''.join(map(str, aggregateFreq(curr)))
            
        return curr
