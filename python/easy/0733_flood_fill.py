from collections import deque

class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        n, m = len(image), len(image[0])
        d = deque([(sr, sc)])
        original = image[sr][sc]
        directions = [(-1, 0), (0, -1), (1, 0), (0, 1)]
        visited = set()
        while len(d) > 0:
            r, c = d.popleft()
            image[r][c] = color
            visited.add((r, c))
            for dr, dc in directions:
                nr, nc = r + dr, c + dc
                if nr >= n or nr < 0 or nc >= m or nc < 0:
                    continue
                if image[nr][nc] != original:
                    continue
                if (nr, nc) in visited:
                    continue
                image[nr][nc] = color
                d.append((nr, nc))
        return image
